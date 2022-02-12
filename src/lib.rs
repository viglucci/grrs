pub fn print_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            let result = writeln!(writer, "{}", line);
            match result {
                Ok(content) => { content }
                Err(error) => {
                    panic!("Failed to write line. Error: {}", error);
                }
            };
        }
    }
}

#[test]
fn print_a_match() {
    let mut result = Vec::new();
    print_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
