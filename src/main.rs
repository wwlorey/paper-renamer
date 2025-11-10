mod filename;
mod llm;
mod pdf;
mod renamer;
mod ui;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "paper-renamer")]
#[command(about = "Automatically rename academic paper PDFs using LLM-extracted metadata", long_about = None)]
struct Args {
    /// Path to the PDF file to rename
    #[arg(value_name = "FILE")]
    file_path: String,

    /// Ollama model to use for metadata extraction
    #[arg(short, long, default_value = "llama3.2:latest")]
    model: String,
}

fn main() {
    if let Err(e) = run() {
        ui::display_error(&format!("{:#}", e));
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Validate that the file exists and is a PDF
    if !args.file_path.ends_with(".pdf") {
        anyhow::bail!("File must be a PDF (*.pdf)");
    }

    let original_filename = renamer::get_filename(&args.file_path)?;

    println!("Analyzing PDF...");

    // Step 1: Extract text from PDF
    let pdf_text = match pdf::extract_pdf_text(&args.file_path) {
        Ok(text) => text,
        Err(e) => {
            ui::display_error(&format!("{:#}", e));

            // Ask if user wants to enter metadata manually
            if ui::ask_manual_metadata()? {
                println!("\nManual metadata entry is not yet implemented.");
                println!("This feature will be added in a future version.");
                anyhow::bail!("Manual metadata entry not available");
            } else {
                ui::display_cancelled();
                return Ok(());
            }
        }
    };

    // Step 2: Extract metadata using LLM
    println!("Extracting metadata using LLM (model: {})...", args.model);
    let metadata = llm::extract_metadata_with_ollama(&pdf_text, &args.model)
        .context("Failed to extract metadata using LLM")?;

    // Display the extracted metadata
    ui::display_metadata(&metadata.first_author, &metadata.year, &metadata.title);

    // Step 3: Generate proposed filename
    let mut proposed_filename = filename::generate_filename(&metadata);

    // Step 4: Get user confirmation
    loop {
        let choice = ui::confirm_rename(&original_filename, &proposed_filename)?;

        match choice {
            ui::UserChoice::Yes => {
                // Validate the filename
                if !filename::validate_filename(&proposed_filename) {
                    ui::display_error("Invalid filename. Please try again.");
                    continue;
                }

                // Perform the rename
                let new_path = renamer::rename_file(&args.file_path, &proposed_filename)
                    .context("Failed to rename file")?;

                ui::display_success(&original_filename, &new_path.display().to_string());
                break;
            }
            ui::UserChoice::No => {
                ui::display_cancelled();
                break;
            }
            ui::UserChoice::Edit => {
                // Let user edit the filename
                proposed_filename = ui::edit_filename(&proposed_filename)?;

                // Ensure it still ends with .pdf
                if !proposed_filename.ends_with(".pdf") {
                    proposed_filename.push_str(".pdf");
                }
            }
        }
    }

    Ok(())
}
