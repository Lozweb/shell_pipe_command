#[cfg(test)]
mod test {
    use crate::modules::shell::{Shell};

    #[test]
    fn cmd_with_single_arg() {
        let out = Shell::new("cat")
            .arg("test_file")
            .result_to_string();

        assert_eq!(out, "hello world !\nother\nline with l")
    }

    #[test]
    fn cmd_with_multiple_args() {
        let out = Shell::new("cat")
            .args(&["test_file", "-b"])
            .result_to_string();

        assert_eq!(out, "     1\thello world !\n     2\tother\n     3\tline with l")
    }

    #[test]
    fn piped_cmd(){
        let out = Shell::new("cat")
            .arg("test_file")
            .pipe("grep")
            .arg("l")
            .result_to_string();

        assert_eq!(out, "hello world !\nline with l\n");
    }

     #[test]
     fn multi_piped_cmd(){
        let out = Shell::new("cat")
            .arg("test_file")
            .pipe("grep")
            .arg("l")
            .pipe("wc")
            .arg("-l")
            .result_to_string();

         assert_eq!(out, "2\n");
     }
}
