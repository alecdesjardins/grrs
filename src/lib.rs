use std::io;

pub fn find_matches(content: &str, patterns: &[String], ignore_case: bool, mut writer: impl io::Write) -> io::Result<()> {
    for line in content.lines() {
        for pattern in patterns {
            let pattern_match = if ignore_case {
                line.to_lowercase().contains(&pattern.to_lowercase())
            } else {
                line.contains(pattern)
            };

            if pattern_match {
                writeln!(writer, "{}", line)?;
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_a_match_ignore_case() -> io::Result<()> {
        let content = "lorem ipsum\ndolor sit amet";
        let patterns = ["IPSUM".to_string()];
        let ignore_case = true;

        let mut result = Vec::new();
        find_matches(&content, &patterns, ignore_case, &mut result)?;

        assert_eq!(result, b"lorem ipsum\n");
        Ok(())
    }

    #[test]
    fn no_match_ignore_case() -> io::Result<()> {
        let content = "lorem ipsum\ndolor sit amet";
        let patterns = ["foo".to_string()];
        let ignore_case = true;

        let mut result = Vec::new();
        find_matches(&content, &patterns, ignore_case, &mut result)?;

        assert_eq!(result, b"");
        Ok(())
    }

    #[test]
    fn finds_match_case_sensitive() -> io::Result<()> {
        let content = "lorem ipsum\ndolor sit amet";
        let patterns = ["ipsum".to_string()];
        let ignore_case = false;

        let mut result = Vec::new();
        find_matches(&content, &patterns, ignore_case, &mut result)?;

        assert_eq!(result, b"lorem ipsum\n");
        Ok(())
    }

    #[test]
    fn no_match_case_sensitive() -> io::Result<()> {
        let content = "lorem ipsum\ndolor sit amet";
        let patterns = ["IPSUM".to_string()];
        let ignore_case = false;

        let mut result = Vec::new();
        find_matches(&content, &patterns, ignore_case, &mut result)?;

        assert_eq!(result, b"");
        Ok(())
    }
}