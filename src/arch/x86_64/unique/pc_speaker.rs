use x86_64::instructions::port::Port;

const PC_SPEAKER_ENABLE: u8 = 0b00000011;

pub fn beep(frequency: u32) {    
    use super::pit::{Command, set_command, send_data_16};
    let command = Command::MODE_3 | Command::ACCESS_16BIT | Command::CHANNEL_2;
    // Calculate the divisor using this formula:
    // https://www.reddit.com/r/osdev/comments/7gorff/pit_and_frequency/dqknp4q?utm_source=share&utm_medium=web2x&context=3
    let divisor = (7159090 + 6 / 2) / (6 * frequency);
    set_command(command);
    send_data_16(2, divisor as u16);
    let mut speaker = Port::new(0x61);
    unsafe {
        let register: u8 = speaker.read();
        if register != register | PC_SPEAKER_ENABLE {
            speaker.write(register | PC_SPEAKER_ENABLE );
        }
    }
}

pub fn stop() {
    let mut speaker = Port::new(0x61);
    unsafe {
        let register: u8 = speaker.read();
        speaker.write(register & !PC_SPEAKER_ENABLE);
    }    
}