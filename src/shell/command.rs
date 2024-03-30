use std::process::Command;

trait Pipe {
    fn pipe(&mut self) -> &mut Command;
}

impl Pipe for Command{
    fn pipe(&mut self) -> &mut Command {
        self
    }
}
pub fn convert_stdout_to_string(stdout: Vec<u8>)-> String {
    String::from_utf8(stdout).expect("cannot convert stdout to string")
}
