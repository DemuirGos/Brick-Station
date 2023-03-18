pub struct Bus {
    devices : Vec<Device>
}

impl Bus {
    pub fn new() -> Bus {
        let mut devices = Vec::new();
        devices.push(Ram::new());
        devices.push(Cpu::new());
        Bus {
            Devices : devices
        }
    }
}

impl Device for Bus {
    fn read(&self, addr: u16) -> u8 {
        self.devices.iter()
            .filter(|device| device.WithinRange(addr))
            .for_each(|device| {
                device.write(addr, value);
            });
    }

    fn write(&self, addr: u16, value: u8) -> unit {
        // pattern match if add is in range of device
        // if so, call device.write
        self.devices.iter()
            .filter(|device| device.WithinRange(addr))
            .for_each(|device| {
                device.write(addr, value);
            });
    }
}