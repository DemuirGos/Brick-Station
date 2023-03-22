use std::{cell::RefCell, rc::Rc};

pub trait DeviceOps {
    fn within_range(&self, _: u16) -> bool {
        true
    }

    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8) -> ();
}