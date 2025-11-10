use anyhow::{Context, Result};
use pdf_extract::extract_text;

/// Extracts text from a PDF file, focusing on the first few pages
/// which typically contain the paper's metadata
pub fn extract_pdf_text(file_path: &str) -> Result<String> {
    // Suppress both stdout and stderr output from pdf_extract crate
    // The crate outputs debug information that clutters the terminal
    let text = suppress_output(|| extract_text(file_path))
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

/// Suppress both stdout and stderr output during function execution
/// This is used to hide all debug output from the pdf_extract crate
fn suppress_output<F, T>(func: F) -> T
where
    F: FnOnce() -> T,
{
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;

        // Save the original stdout and stderr
        let stdout_fd = std::io::stdout().as_raw_fd();
        let stderr_fd = std::io::stderr().as_raw_fd();
        let original_stdout = unsafe { libc::dup(stdout_fd) };
        let original_stderr = unsafe { libc::dup(stderr_fd) };

        // Redirect both stdout and stderr to /dev/null
        let dev_null = std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/null")
            .expect("Failed to open /dev/null");

        unsafe {
            libc::dup2(dev_null.as_raw_fd(), stdout_fd);
            libc::dup2(dev_null.as_raw_fd(), stderr_fd);
        }

        // Execute the function
        let result = func();

        // Restore original stdout and stderr
        unsafe {
            libc::dup2(original_stdout, stdout_fd);
            libc::dup2(original_stderr, stderr_fd);
            libc::close(original_stdout);
            libc::close(original_stderr);
        }

        result
    }

    #[cfg(not(unix))]
    {
        // On non-Unix systems (Windows), just execute the function
        // Output suppression is more complex on Windows
        func()
    }
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
