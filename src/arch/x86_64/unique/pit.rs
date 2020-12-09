use bitflags::bitflags;

bitflags! {
    pub struct Command: u8 {
        const BCD_MODE = 0b00000001;
        const MODE_0 = 0b00000000;
        const MODE_1 = 0b00000010;
        const MODE_2 = 0b00000100;
        const MODE_3 = 0b00000110;
        const MODE_4 = 0b00001000;
        const MODE_5 = 0b00001010;
        const MODE_2_ALTERNATE = 0b00001100;
        const MODE_3_ALTERNATE = 0b00001110;
        const LATCH_COUNT_VALUE = 0b00000000;
        const ACCESS_LOW = 0b00010000;
        const ACCESS_HIGH = 0b00100000;
        const ACCESS_16BIT = 0b00110000;
        const CHANNEL_0 = 0b00000000;
        const CHANNEL_1 = 0b01000000;
        const CHANNEL_2 = 0b10000000;
        const READ_BACK = 0b11000000;
    }
}

pub fn set_command(command: Command) {
    let mut command_register = x86_64::instructions::port::Port::new(0x43);
    unsafe { command_register.write(command.bits()) };
}

pub fn send_data_8(channel: u8, data: u8) -> Result<(), ()> {
    if channel <= 2 {
        let port_number = (0x40 + channel) as u16;
        let mut port = x86_64::instructions::port::Port::new(port_number);
        unsafe { port.write(data) };
        Ok(())
    } else {
        Err(())
    }
}

pub fn send_data_16(channel: u8, data: u16) -> Result<(), ()> {
    send_data_8(channel, data as u8)?;
    send_data_8(channel, (data >> 8) as u8)?;
    Ok(())
}