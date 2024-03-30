use crate::shell::shell::Shell;

mod shell;
fn main() {
    //demo
    let pwd = Shell::new("pwd").result_to_string();
    println!("{}", pwd);

    let multi_args = Shell::new("cat")
        .args(&["test_file", "-b"])
        .result_to_string();

    println!("{}", multi_args);

    let pipe_command = Shell::new("cat")
        .arg("test_file")
        .pipe("grep")
        .arg("l")
        .result_to_string();

    println!("{}", pipe_command)
}


