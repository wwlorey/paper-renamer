use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaperMetadata {
    pub first_author: String,
    pub year: String,
    pub title: String,
}

#[derive(Debug, Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    format: String,
}

#[derive(Debug, Deserialize)]
struct OllamaResponse {
    response: String,
}

/// Extract paper metadata using Ollama LLM
pub fn extract_metadata_with_ollama(pdf_text: &str, model: &str) -> Result<PaperMetadata> {
    let client = Client::new();

    let prompt = format!(
        r#"You are analyzing the first page of an academic paper. Extract the following information and respond ONLY with valid JSON in this exact format:
{{
  "first_author": "LastName",
  "year": "YYYY",
  "title": "Full Paper Title"
}}

Rules:
- For first_author: extract ONLY the last name of the first author
- For year: extract the publication year as a 4-digit number
- For title: extract the complete paper title
- Respond with ONLY the JSON, no other text

Paper text:
{}

JSON response:"#,
        pdf_text
    );

    let request = OllamaRequest {
        model: model.to_string(),
        prompt,
        stream: false,
        format: "json".to_string(),
    };

    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .context("Failed to send request to Ollama. Make sure Ollama is running (try: ollama serve)")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Ollama API returned error status: {}",
            response.status()
        );
    }

    let ollama_response: OllamaResponse = response
        .json()
        .context("Failed to parse Ollama response")?;

    let metadata: PaperMetadata = serde_json::from_str(&ollama_response.response)
        .context("Failed to parse metadata from LLM response. The LLM may not have returned valid JSON.")?;

    // Validate the extracted metadata
    if metadata.first_author.is_empty() || metadata.year.is_empty() || metadata.title.is_empty() {
        anyhow::bail!("LLM failed to extract all required metadata fields");
    }

    Ok(metadata)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_parsing() {
        let json = r#"{"first_author": "Smith", "year": "2020", "title": "Deep Learning"}"#;
        let metadata: PaperMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.first_author, "Smith");
        assert_eq!(metadata.year, "2020");
        assert_eq!(metadata.title, "Deep Learning");
    }
}
