// PMS5003 uses these UART settings:
// - default baud rate: 9600bps
// - check bit: None
// - stop bit: 1 bit
//
// This is a bare-bones USART driver that supports these requirements. The
// hardware offers a lot of other features that are unsupported by this driver.

use crate::register::{Register, RegisterAddr};

pub struct Serial {
    ubrrn: u16
}

// Frequency of the system clock; the USART on the chip scales
// this clock down to synchronize with the serial baud rate.
// The Arduino Mega 2650 board ships with an external 16MHz system clock.
const CLK: u32 = 16_000_000;

impl Serial {
    pub const fn new(baud_rate: u32) -> Serial {
        // this formula was taken from the atmega2650 datasheet
        let ubrrn = (CLK / (16 * baud_rate) - 1) as u16;
        Serial { ubrrn }
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
        let ubrrh = Register::new(RegisterAddr::UBRR0H);
        let ubrrl = Register::new(RegisterAddr::UBRR0L);
        ubrrh.write((self.ubrrn >> 8) as u8);
        ubrrl.write(self.ubrrn as u8);

        // USART status and control registers
        let ucsrb = Register::new(RegisterAddr::UCSR0B);
        let ucsrc = Register::new(RegisterAddr::UCSR0C);

        // flags to control rx/tx on the USART
        let rx_enable: u8 = 0b00010000;
        let tx_enable: u8 = 0b00001000;

        // These flags tell the hardware to send interrupts for async handling
        // of serial I/O. Could be interesting but completely unecessary for now
        // let rx_complete_interrupt_enable = 0b10000000;
        // let tx_complete_interrupt_enable = 0b01000000;

        // aka uscz (usart character size)
        // there are only 3 bits to represent this quantity
        let data_bits: u8 = 8;
        let data_bits: u8 = data_bits & 0b00000111;

        // aka upm (usart parity mode)
        // let parity_bit = 0b00100000;      // enable partiy check
        // let parity_bit_odd = 0b00010000;  // even or odd parity check

        // aka usbs (usart stop bit select)
        // let stop_bits = 1;  // \in {1, 2}

        // get the most significant data bit to put in ucsrb
        let data_bit_h: u8 = data_bits >> 2;
        // get the two least significant data bits to put in ucsrc
        let data_bit_l: u8 = data_bits & 0b00000011;

        ucsrb.write(rx_enable | tx_enable | (data_bit_h << 1));
        ucsrc.write(data_bit_l << 1);
    }

    // assumptions: init() must be called before tx()
    // TODO: express this with the type system so not calling init() is a
    // compile time error
    pub fn tx(&self, buf: &[u8]) {
        let udr = Register::new(RegisterAddr::UDR0);
        for byte in buf {
            self.wait_data_register_empty();
            udr.write(*byte);
        }
    }

    // assumptions: init() must be called before rx()
    // TODO: express this with the type system so not calling init() is a
    // compile error
    pub fn rx(&self, buf: &mut [u8]) {
        let udr = Register::new(RegisterAddr::UDR0);
        for i in 0..buf.len() {
            self.wait_data_register_empty();
            buf[i] = udr.read();
        }
    }

    // Blocks until the data register is ready for tx/rx
    fn wait_data_register_empty(&self) {
        let ucsra = Register::new(RegisterAddr::UCSR0A);
        let data_empty: u8 = 0b00100000;  // UDRE flag is set when empty
        loop {
            if ucsra.read() & data_empty != 0 {
                break;
            }
        }
    }

    // TODO: add some helpers to read RX/TX complete bits so we can busy-wait on
    // them until we're ready to read data out. I think these are in ucsra.
}
