use crate::shell::shell_cmd::shell_cmd::*;

mod shell;


fn main() {
    let out = cat_file("test_file");
    let content = convert_stdout_to_string(out.stdout);
    println!("{}", content);

    let out = pwd();
    let content = convert_stdout_to_string(out.stdout);
    println!("{}", content);

    let out = cat_file_piped("test_file");
    let content = convert_stdout_to_string(out.stdout);
    println!("{}", content);
}


