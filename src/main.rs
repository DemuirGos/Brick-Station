use debugger::disassembler::State;

pub mod hardware;
pub mod debugger;
fn main() {
    State::start().unwrap();
}