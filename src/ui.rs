use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

#[derive(Debug, PartialEq)]
pub enum UserChoice {
    Yes,
    No,
    Edit,
}

/// Prompt the user to confirm the rename operation
/// Returns the user's choice: Yes, No, or Edit
pub fn confirm_rename(original: &str, proposed: &str) -> Result<UserChoice> {
    println!("\nProposed filename: {}", proposed);
    println!();

    let choices = vec!["Yes - rename the file", "No - cancel", "Edit - modify filename"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(&format!(
            "Would you like to rename '{}' to '{}'?",
            original, proposed
        ))
        .items(&choices)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => UserChoice::Yes,
        1 => UserChoice::No,
        2 => UserChoice::Edit,
        _ => unreachable!(),
    })
}

/// Prompt the user to edit the proposed filename
/// Returns the edited filename
/// The current filename is pre-filled for editing
pub fn edit_filename(proposed: &str) -> Result<String> {
    println!("\nEdit the filename below (current filename is pre-filled):");

    let edited: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Filename")
        .default(proposed.to_string())
        .allow_empty(false)
        .interact_text()?;

    Ok(edited)
}

/// Ask if the user wants to enter metadata manually
pub fn ask_manual_metadata() -> Result<bool> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to enter metadata manually?")
        .default(false)
        .interact()
        .map_err(|e| e.into())
}

/// Display metadata extracted from the PDF
pub fn display_metadata(author: &str, year: &str, title: &str) {
    println!("\nExtracted metadata:");
    println!("  - First Author: {}", author);
    println!("  - Year: {}", year);
    println!("  - Title: {}", title);
}

/// Display success message
pub fn display_success(old_name: &str, new_name: &str) {
    println!("\n✓ File renamed successfully!");
    println!("  {} -> {}", old_name, new_name);
}

/// Display cancellation message
pub fn display_cancelled() {
    println!("\nOperation cancelled.");
}

/// Display error message
pub fn display_error(error: &str) {
    eprintln!("\n⚠ Error: {}", error);
}
