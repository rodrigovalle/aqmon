use core::ptr;

// pulled these register mappings from the atmega2560 datasheet
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Register {
    UDR3   = 0x136,  // USART3 I/O data register
    UBRR3H = 0x135,  // USART3 baud rate register high byte
    UBRR3L = 0x134,  // USART3 baud rate register low byte
    UCSR3C = 0x132,  // USART3 configuration
    UCSR3B = 0x131,  // USART3 configuration
    UCSR3A = 0x130,  // USART3 configuration

    UDR2   = 0xD6,  // USART2 I/O data register
    UBRR2H = 0xD5,  // USART2 baud rate register high byte
    UBRR2L = 0xD4,  // USART2 baud rate register low byte
    UCSR2C = 0xD2,  // USART2 configuration
    UCSR2B = 0xD1,  // USART2 configuration
    UCSR2A = 0xD0,  // USART2 configuration

    UDR1   = 0xCE,  // USART1 I/O data register
    UBRR1H = 0xCD,  // USART1 baud rate register high byte
    UBRR1L = 0xCC,  // USART1 baud rate register low byte
    UCSR1C = 0xCA,  // USART1 configuration
    UCSR1B = 0xC9,  // USART1 configuration
    UCSR1A = 0xC8,  // USART1 configuration

    UDR0   = 0xC6,  // USART0 I/O data register
    UBRR0H = 0xC5,  // USART0 baud rate register high byte
    UBRR0L = 0xC4,  // USART0 baud rate register low byte
    UCSR0C = 0xC2,  // USART0 configuration
    UCSR0B = 0xC1,  // USART0 configuration
    UCSR0A = 0xC0,  // USART0 configuration
}

impl Register {
    pub fn read(&self) -> u8 {
        let addr = *self as u16;
        unsafe { ptr::read_volatile(addr as *mut u8) }
    }

    pub fn write(&self, val: u8) {
        let addr = *self as u16;
        unsafe { ptr::write_volatile(addr as *mut u8, val) }
    }
}
