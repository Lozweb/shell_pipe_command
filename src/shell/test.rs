#[cfg(test)]
mod test {
    use std::process::Command;
    use crate::shell::*;

    #[test]
    fn print_content_file(){
        let out = shell_cmd::cat_file("test_file");
        let content = command::convert_stdout_to_string(out.stdout);
        assert_eq!(content, "hello world !")
    }

    #[test]
    fn print_content_file_piped(){
        let out = shell_cmd::cat_file_piped("test_file");
        let content = command::convert_stdout_to_string(out.stdout);
        assert_eq!(content, "hello world !")
    }

    #[test]
    fn piped_cmd(){
        let grep_from_file = Command::new("cat")
            .arg("test_file")
            .pipe()
    }
}
