#[cfg(test)]
mod test {
    use crate::shell::command::{Shell};

    #[test]
    fn piped_cmd(){

        let grep_from_file = Shell::new("cat")
            .arg("test_file")
            .pipe("grep")
            .arg("l")
            .result();

        println!("{}", grep_from_file);
    }
}
