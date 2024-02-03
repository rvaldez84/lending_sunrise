use gtest::{Log, Program, System};
use std::fs;

#[test]
fn test() {
    let system = System::new();

    system.init_logger();
    let program = Program::current(&system);
}
