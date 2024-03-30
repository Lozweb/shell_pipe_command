use std::process::{Child, Command, Output, Stdio};

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

    pub fn args(&mut self, a: Vec<&str>) -> &mut Self {
        self.shell.args(a);
        self
    }

    pub fn pipe(&mut self, cmd:&str) -> Shell {
        let child = self.shell.spawn().expect("failed to spawn command");
        let out = child.stdout.expect("failed to open child stdout");
        let mut new_cmd = Shell { shell: Command::new(cmd)};
        new_cmd.shell.stdin(Stdio::from(out));
        new_cmd
    }

    pub fn output(&mut self) -> Output {
        self
            .exec()
            .wait_with_output()
            .expect("failed with wait output")
    }

    pub fn exec(&mut self)-> Child {
        self.shell.spawn().expect("failed to spawn command")
    }

    pub fn result(&mut self) -> String {
        String::from_utf8(self.output().stdout).expect("cannot convert stdout to string")
    }

}


