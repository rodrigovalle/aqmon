// PMS5003 uses these UART settings:
// - default baud rate: 9600bps
// - check bit: None
// - stop bit: 1 bit
//
// This is a bare-bones USART driver that supports these requirements. The
// hardware offers a lot of other features that are unsupported by this driver.

use crate::register::Register;

// Frequency of the system clock; the USART on the chip scales
// this clock down to synchronize with the serial baud rate.
// The Arduino Mega 2650 board ships with an external 16MHz system clock.
const CLK: u32 = 16_000_000;

struct SerialRegisterBlock {
    udr: Register,
    ubrrh: Register,
    ubrrl: Register,
    ucsra: Register,
    ucsrb: Register,
    ucsrc: Register,
}

const SERIAL_REGISTER_BLOCKS: [SerialRegisterBlock; 4] = [
    SerialRegisterBlock {
        udr: Register::UDR0,
        ubrrh: Register::UBRR0H,
        ubrrl: Register::UBRR0L,
        ucsra: Register::UCSR0A,
        ucsrb: Register::UCSR0B,
        ucsrc: Register::UCSR0C,
    },
    SerialRegisterBlock {
        udr: Register::UDR1,
        ubrrh: Register::UBRR1H,
        ubrrl: Register::UBRR1L,
        ucsra: Register::UCSR1A,
        ucsrb: Register::UCSR1B,
        ucsrc: Register::UCSR1C,
    },
    SerialRegisterBlock {
        udr: Register::UDR2,
        ubrrh: Register::UBRR2H,
        ubrrl: Register::UBRR2L,
        ucsra: Register::UCSR2A,
        ucsrb: Register::UCSR2B,
        ucsrc: Register::UCSR2C,
    },
    SerialRegisterBlock {
        udr: Register::UDR3,
        ubrrh: Register::UBRR3H,
        ubrrl: Register::UBRR3L,
        ucsra: Register::UCSR3A,
        ucsrb: Register::UCSR3B,
        ucsrc: Register::UCSR3C,
    },
];

pub struct Serial<'a> {
    ubrr: u16,
    registers: &'a SerialRegisterBlock,
}

impl<'a> Serial<'a> {
    pub const fn new(port: u8, baud_rate: u32) -> Serial<'a> {
        // this formula was taken from the atmega2650 datasheet
        let ubrr = (CLK / (16 * baud_rate) - 1) as u16;
        let registers = &SERIAL_REGISTER_BLOCKS[port as usize];
        Serial { ubrr, registers }
    }

    // to intialize the UART we need to
    // - set the baud rate
    // - set the frame format
    // - enable Tx/Rx
    //
    // Registers (roughly):
    // - UBRRnH: four most significant bits of baud rate
    // - UBRRnL: 8 least significant bits of baud rate
    // - UCSRB: enable/disable Tx/Rx
    // - UCSRC: specify frame format
    //
    // Flags
    // - TXCn flag: check if transmitter has completed all transfers
    // - RXC flag: check if no unread data in the receive buffer
    pub fn init(&self) {
        // TODO:
        // datasheet also recommends zeroing the power reduction bit PRUSART{N}
        // this might have been set if we disabled the USART previously to save
        // power.

        // set the baud rate
        self.registers.ubrrh.write((self.ubrr >> 8) as u8);
        self.registers.ubrrl.write(self.ubrr as u8);

        // flags to control rx/tx on the USART
        let rx_enable: u8 = 0b00010000;
        let tx_enable: u8 = 0b00001000;

        // These flags tell the hardware to send interrupts for async handling
        // of serial I/O. Could be interesting but completely unecessary for now
        // let rx_complete_interrupt_enable = 0b10000000;
        // let tx_complete_interrupt_enable = 0b01000000;

        // aka upm (usart parity mode)
        // let parity_bit = 0b00100000;      // enable partiy check
        // let parity_bit_odd = 0b00010000;  // even or odd parity check

        // aka usbs (usart stop bit select)
        // let stop_bits = 1;

        // aka uscz (usart character size)
        // set manually from the data sheet to 8-bit frames
        let data_bits: u8 = 0b00000110;  // 8-bit frames

        self.registers.ucsrb.write(rx_enable | tx_enable);
        self.registers.ucsrc.write(data_bits);
    }

    // assumptions: init() must be called before tx()
    // TODO: express this with the type system so not calling init() is a
    // compile time error
    pub fn tx(&self, buf: &[u8]) {
        let data_empty: u8 = 0b00100000;  // UDRE flag is set when empty

        for byte in buf {
            loop {
                if self.registers.ucsra.read() & data_empty != 0 {
                    break;
                }
            }
            self.registers.udr.write(*byte);
        }
    }

    // assumptions: init() must be called before rx()
    // TODO: express this with the type system so not calling init() is a
    // compile error
    pub fn rx(&self, buf: &mut [u8]) {
        let rx_complete = 0b10000000;

        for i in 0..buf.len() {
            loop {
                if self.registers.ucsra.read() & rx_complete != 0 {
                    break;
                }
            }

            // The runtime bounds checking can cause this to panic, and
            // panicking causes core::fmt to be linked in which has a hefty
            // memory cost:
            //
            //   buf[i] = udr.read();
            //
            // Avoid panicking and use the get() methods instead.
            if let Some(byte) = buf.get_mut(i) {
                *byte = self.registers.udr.read();
            }
        }
    }

    // TODO: add some helpers to read RX/TX complete bits so we can busy-wait on
    // them until we're ready to read data out. I think these are in ucsra.
}
