use std::process::{Command, Output, Stdio};

pub fn cat_file(file:&str) -> Output{
    Command::new("cat")
        .arg(file)
        .output()
        .expect("cannot open file")
}

pub fn cat_file_piped(file:&str) -> Output{
    let cat_child = Command::new("cat")
        .arg(file)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start cat command");

    let cat_out = cat_child.stdout.expect("failed to open cat stdout");

    let grep_child = Command::new("grep")
        .arg("l")
        .stdin(Stdio::from(cat_out))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start grep command");

    let out = grep_child.wait_with_output().expect("failed to wait on grep");
    out
}

pub fn pwd() -> Output{
    Command::new("pwd")
        .output()
        .expect("cannot open file")
}



