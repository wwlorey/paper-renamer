use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Rename a file to the new filename
/// The new file will be in the same directory as the original file
pub fn rename_file(original_path: &str, new_filename: &str) -> Result<PathBuf> {
    let original = Path::new(original_path);

    // Validate that the original file exists
    if !original.exists() {
        anyhow::bail!("Original file does not exist: {}", original_path);
    }

    if !original.is_file() {
        anyhow::bail!("Path is not a file: {}", original_path);
    }

    // Get the directory of the original file
    let parent_dir = original
        .parent()
        .context("Failed to get parent directory")?;

    // Create the new path in the same directory
    let new_path = parent_dir.join(new_filename);

    // Check if the target file already exists
    if new_path.exists() {
        anyhow::bail!(
            "Target file already exists: {}. Choose a different name.",
            new_path.display()
        );
    }

    // Perform the rename
    fs::rename(original, &new_path).context("Failed to rename file")?;

    Ok(new_path)
}

/// Get just the filename from a path
pub fn get_filename(path: &str) -> Result<String> {
    let path = Path::new(path);
    let filename = path
        .file_name()
        .context("Failed to get filename from path")?
        .to_str()
        .context("Filename contains invalid UTF-8")?;

    Ok(filename.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_get_filename() {
        assert_eq!(
            get_filename("/path/to/file.pdf").unwrap(),
            "file.pdf"
        );
        assert_eq!(get_filename("file.pdf").unwrap(), "file.pdf");
    }

    #[test]
    fn test_rename_file() {
        let temp_dir = TempDir::new().unwrap();
        let original_path = temp_dir.path().join("original.pdf");
        File::create(&original_path).unwrap();

        let new_path = rename_file(
            original_path.to_str().unwrap(),
            "renamed.pdf",
        )
        .unwrap();

        assert!(new_path.exists());
        assert!(!original_path.exists());
        assert_eq!(new_path.file_name().unwrap(), "renamed.pdf");
    }
}
