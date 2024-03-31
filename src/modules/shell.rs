use std::io;
use std::process::{Child, Command, ExitStatus, Output, Stdio};

pub struct Shell{
    pub(crate) shell: Command
}

impl Shell {
    pub fn new(cmd: &str) -> Shell {
        let mut command = Shell { shell: Command::new(cmd) };
        command.shell.stdout(Stdio::piped());
        command.shell.stderr(Stdio::piped());
        command
    }
    pub fn arg(&mut self, arg: &str) -> &mut Self {
        self.shell.arg(arg);
        self
    }

    pub fn args(&mut self, a: &[&str]) -> &mut Self {
        self.shell.args(a);
        self
    }

    pub fn pipe(&mut self, cmd:&str) -> Shell {
        let child = self.shell.spawn().expect("failed to spawn command");
        let out = child.stdout.expect("failed to open child stdout");
        let mut new_cmd = Shell { shell: Command::new(cmd)};
        new_cmd.shell.stdin(Stdio::from(out));
        new_cmd.shell.stdout(Stdio::piped());
        new_cmd.shell.stderr(Stdio::piped());
        new_cmd
    }

    pub fn result_to_string(&mut self) -> String {

        let output = self.output();

        if output.stdout.len() > 0 {
            Self::bytes_to_string(output.stdout)
        } else {
            Self::bytes_to_string(output.stderr)
        }

    }

    fn bytes_to_string(out:Vec<u8>) -> String {
        String::from_utf8(out).unwrap_or_else(|err| err.to_string())
    }

    fn output(&mut self) -> Output {

        match self.exec() {
            Ok(child) => child.wait_with_output().expect("failed wait with output"),
            Err(err) => Output {
                status: ExitStatus::default(),
                stdout: vec![],
                stderr: Vec::from(err.to_string().as_bytes()),
            }
         }
    }
    //todo wait with output handler
    fn exec(&mut self)-> io::Result<Child> {
        self.shell.spawn()
    }

}


