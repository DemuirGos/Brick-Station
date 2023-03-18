pub trait Device {
    fn withinRange(&self, addr: u16) -> bool;
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, value: u8) -> ();
}