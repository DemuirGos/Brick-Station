use debugger::debugger::App;

pub mod hardware;
pub mod debugger;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    App::start(args[1].clone()).unwrap();
}