#[derive(Clone, Copy)]
pub enum Originator {
    Cpu, Ppu
}
pub trait DeviceOps {
    fn within_range(&self, _: u16) -> bool {
        true
    }

    fn read(&self, addr: u16, reader: Originator) -> u8;
    fn write(&mut self, addr: u16, value: u8, reader: Originator) -> ();
}