use super::{interfaces::Device, ram::Ram, cpu::Cpu};

pub struct Bus(pub Vec<Box<Device>>);

impl Bus {
    pub fn new() -> Bus {
        let mut devices = Vec::<Box<Device>>::new();
        devices.push(Box::new(Cpu::new()));
        devices.push(Box::new(Ram::new()));
        Bus(devices)
    }
}

impl Device for Bus {
    fn read(&self, addr: u16) -> u8 {
        self.0.iter()
            .filter(|device| device.withinRange(addr))
            .map(|device| device.read(addr))
            .nth(0).unwrap()
    }

    fn write(&mut self, addr: u16, value: u8) -> () {
        self.0.iter_mut()
            .filter(|device| device.withinRange(addr))
            .for_each(|device| {
                device.write(addr, value);
            });
    }
}