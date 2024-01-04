use std::io;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl io::Write) -> io::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_a_match() -> io::Result<()> {
        let content = "lorem ipsum\ndolor sit amet";
        let pattern = "ipsum";
        let mut result = Vec::new();
        find_matches(&content, &pattern, &mut result)?;

        let result = String::from_utf8(result).expect("Not UTF-8");
        assert_eq!(result, "lorem ipsum\n");
        Ok(())
    }

    #[test]
    fn no_match() -> io::Result<()> {
        let content = "lorem ipsum\ndolor sit amet";
        let pattern = "foo";
        let mut result = Vec::new();
        find_matches(&content, &pattern, &mut result)?;

        let result = String::from_utf8(result).expect("Not UTF-8");
        assert_eq!(result, "");
        Ok(())
    }
}