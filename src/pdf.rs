use anyhow::{Context, Result};
use pdf_extract::extract_text;

/// Extracts text from a PDF file, focusing on the first few pages
/// which typically contain the paper's metadata
pub fn extract_pdf_text(file_path: &str) -> Result<String> {
    let text = extract_text(file_path)
        .context("Failed to extract text from PDF")?;

    if text.trim().is_empty() {
        anyhow::bail!("No text could be extracted from the PDF. The file may be a scanned image.");
    }

    // Return first ~3000 characters which should contain metadata
    // This reduces the amount of text we need to send to the LLM
    let truncated = if text.len() > 3000 {
        &text[..3000]
    } else {
        &text
    };

    Ok(truncated.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pdf_text() {
        // This test requires a sample PDF file
        // In a real implementation, we would add a test PDF to the repo
    }
}
