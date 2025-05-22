use crate::enums::mode::Mode;

pub struct Console {
    ip: String,
    port: u16,
    mode: Mode
}

impl Console {
    pub fn new() -> Self {
        Console {ip: "".to_string(), port: 0, mode: Mode::Unknown}
    }

    // Coming soon
}