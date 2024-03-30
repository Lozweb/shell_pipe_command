#[cfg(test)]
mod test {
    use crate::shell::shell::{Shell};

    #[test]
    fn piped_cmd(){

        let grep_from_file = Shell::new("cat")
            .arg("test_file")
            .pipe("grep")
            .arg("l")
            .result_to_string();

        println!("{}", grep_from_file);
    }
}
