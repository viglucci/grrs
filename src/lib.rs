use std::str;
use serde_json::{Value, json};

pub fn print_matches(content: &str, pattern: &str, mut writer: impl std::io::Write, json: &bool) {
    for line in content.lines() {
        if line.contains(pattern) {
            let mut string: String = String::from(line);
            if *json {
                let v: Value = json!({
                    "type": "line",
                    "content": line,
                });
                string = v.to_string();
            }
            let result = writeln!(writer, "{}", string);
            match result {
                Ok(content) => { content }
                Err(error) => {
                    panic!("Failed to write line. Error: {}", error);
                }
            };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_str::assert_str_eq;

    fn buf_to_str(buf: &Vec<u8>) -> &str {
        let str = match str::from_utf8(&buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        return str;
    }


    #[test]
    fn print_a_match() {
        let mut buf = Vec::new();
        let input = "lorem ipsum\ndolor sit amet";
        print_matches(input, "lorem", &mut buf, &false);
        let actual = buf_to_str(&buf);
        let expected = "lorem ipsum";
        assert_str_eq!(expected, actual, "Responses should be equal");
    }

    #[test]
    fn print_a_match_as_json() {
        let mut buf: Vec<u8> = Vec::new();
        let input = "lorem ipsum\ndolor sit amet";
        print_matches(input, "lorem", &mut buf, &true);
        let actual = buf_to_str(&buf);
        let expected: String = String::from("{\"content\":\"lorem ipsum\",\"type\":\"line\"}");
        assert_str_eq!(expected, actual, "Responses should be equal");
    }
}
