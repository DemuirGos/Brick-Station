use debugger::debugger::State;

pub mod hardware;
pub mod debugger;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    State::start(args[1].clone()).unwrap();
}