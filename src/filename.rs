use crate::llm::PaperMetadata;

/// Generate a sanitized filename from paper metadata
/// Format: <first-author-last-name><year><paper-title>.pdf
/// Rules:
/// - All lowercase
/// - Dashes (-) instead of spaces
/// - No special characters
pub fn generate_filename(metadata: &PaperMetadata) -> String {
    let author = sanitize(&metadata.first_author);
    let year = sanitize(&metadata.year);
    let title = sanitize(&metadata.title);

    format!("{}-{}-{}.pdf", author, year, title)
}

/// Sanitize a string according to the naming convention:
/// - Convert to lowercase
/// - Replace spaces with dashes
/// - Remove special characters (keep only alphanumeric and dashes)
/// - Remove multiple consecutive dashes
/// - Trim leading/trailing dashes
fn sanitize(s: &str) -> String {
    s.to_lowercase()
        // Replace spaces and underscores with dashes
        .replace(' ', "-")
        .replace('_', "-")
        // Remove all characters except alphanumeric and dashes
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        // Replace multiple consecutive dashes with single dash
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Validate that a filename is safe and doesn't contain path traversal attempts
pub fn validate_filename(filename: &str) -> bool {
    !filename.contains("..")
        && !filename.contains('/')
        && !filename.contains('\\')
        && !filename.is_empty()
        && filename.ends_with(".pdf")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize() {
        assert_eq!(sanitize("Hello World"), "hello-world");
        assert_eq!(sanitize("Test_File-Name"), "test-file-name");
        assert_eq!(sanitize("Special!@#$%Chars"), "specialchars");
        assert_eq!(sanitize("Multiple   Spaces"), "multiple-spaces");
        assert_eq!(sanitize("Vaswani"), "vaswani");
    }

    #[test]
    fn test_generate_filename() {
        let metadata = PaperMetadata {
            first_author: "Vaswani".to_string(),
            year: "2017".to_string(),
            title: "Attention Is All You Need".to_string(),
        };

        let filename = generate_filename(&metadata);
        assert_eq!(filename, "vaswani-2017-attention-is-all-you-need.pdf");
    }

    #[test]
    fn test_validate_filename() {
        assert!(validate_filename("valid-filename.pdf"));
        assert!(!validate_filename("../etc/passwd.pdf"));
        assert!(!validate_filename("path/to/file.pdf"));
        assert!(!validate_filename(""));
        assert!(!validate_filename("no-extension"));
    }
}
