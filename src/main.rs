use crate::modules::shell::Shell;

mod modules;
fn main() {
    //demo
    let cmd_error = Shell::new("cat")
        .args(&["test_file", "-b"])
        .pipe("toto")
        .arg("-t")
        .result_to_string();
    
    println!("{}", cmd_error)
}


