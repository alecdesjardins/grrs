use std::io;

pub fn find_matches(content: &str, patterns: &[String], mut writer: impl io::Write) -> io::Result<()> {
    for line in content.lines() {
        if patterns.iter().any(|pattern| line.contains(pattern)) {
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
        let patterns = ["ipsum".to_string()];

        let mut result = Vec::new();
        find_matches(&content, &patterns, &mut result)?;

        assert_eq!(result, b"lorem ipsum\n");
        Ok(())
    }

    #[test]
    fn no_match() -> io::Result<()> {
        let content = "lorem ipsum\ndolor sit amet";
        let patterns = ["foo".to_string()];

        let mut result = Vec::new();
        find_matches(&content, &patterns, &mut result)?;

        assert_eq!(result, b"");
        Ok(())
    }
}