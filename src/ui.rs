use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, PartialEq)]
pub enum UserChoice {
    Yes,
    No,
    Edit,
    EditAuthor,
    EditYear,
    EditTitle,
}

/// Prompt the user to confirm the rename operation
/// Returns the user's choice: Yes, No, Edit, EditAuthor, EditYear, or EditTitle
pub fn confirm_rename(original: &str, proposed: &str) -> Result<UserChoice> {
    println!("\nProposed filename: {}", proposed);
    println!();

    let choices = vec![
        "Yes - rename the file",
        "No - cancel",
        "Edit filename - modify the complete filename",
        "Edit author - change the author name",
        "Edit year - change the publication year",
        "Edit title - change the paper title",
    ];

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
        3 => UserChoice::EditAuthor,
        4 => UserChoice::EditYear,
        5 => UserChoice::EditTitle,
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
        .with_initial_text(proposed)
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

/// Prompt the user to edit the author
/// Returns the edited author name
/// The current author is pre-filled for editing
pub fn edit_author(current: &str) -> Result<String> {
    println!("\nEdit the author (last name only):");

    let edited: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Author")
        .with_initial_text(current)
        .allow_empty(false)
        .interact_text()?;

    Ok(edited)
}

/// Prompt the user to edit the year
/// Returns the edited year
/// The current year is pre-filled for editing
pub fn edit_year(current: &str) -> Result<String> {
    println!("\nEdit the publication year:");

    let edited: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Year")
        .with_initial_text(current)
        .allow_empty(false)
        .interact_text()?;

    Ok(edited)
}

/// Prompt the user to edit the title
/// Returns the edited title
/// The current title is pre-filled for editing
pub fn edit_title(current: &str) -> Result<String> {
    println!("\nEdit the paper title:");

    let edited: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Title")
        .with_initial_text(current)
        .allow_empty(false)
        .interact_text()?;

    Ok(edited)
}

/// Create a spinner with a custom message
/// Returns a ProgressBar that should be finished when the operation completes
pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"
            ])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner
}

/// Finish a spinner with a success message
pub fn finish_spinner(spinner: ProgressBar, message: &str) {
    spinner.finish_with_message(format!("✓ {}", message));
}
