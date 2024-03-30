use std::process::{Output};
use crate::shell::command::Shell;

pub fn test_shell_struct() -> Output{
    Shell::new("cat")
        .arg("test_file")
        .pipe("grep")
        .arg("l")
        .shell
        .spawn()
        .expect("failed to execute child process")
        .wait_with_output()
        .expect("failed to wait process output")
}



