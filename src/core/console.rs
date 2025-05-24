use crate::enums::mode::Mode;

pub struct Console {
    ip: String,
    port: u16,
    mode: Mode
}

impl Console {
    pub fn new() -> Self {
        

        Console {ip: String::new(), port: 0, mode: Mode::Unknown}
    }

    // Setters
    fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    // Getters
    fn get_ip(&self) -> &String {
        &self.ip
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_mode(&self) -> &Mode {
        &self.mode
    }
    
    pub fn run(&self) {
        // TODO: Implement the console
    }
}