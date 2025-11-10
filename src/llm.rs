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

#[derive(Debug, Deserialize)]
struct RunningModel {
    name: String,
}

#[derive(Debug, Deserialize)]
struct RunningModelsResponse {
    models: Vec<RunningModel>,
}

#[derive(Debug, Deserialize)]
struct AvailableModel {
    name: String,
}

#[derive(Debug, Deserialize)]
struct AvailableModelsResponse {
    models: Vec<AvailableModel>,
}

/// Detect which Ollama model to use
/// First checks for running models, then falls back to available models
pub fn detect_ollama_model() -> Result<String> {
    let client = Client::new();

    // Try to connect to Ollama first
    let health_check = client
        .get("http://localhost:11434/api/tags")
        .send();

    if health_check.is_err() {
        anyhow::bail!(
            "Cannot connect to Ollama. Please start Ollama first:\n\n\
            1. If Ollama is not installed, visit: https://ollama.ai\n\
            2. If Ollama is installed, start it with: ollama serve\n\
            3. Then pull a model, for example: ollama pull llama3.2"
        );
    }

    // First, try to find a running model
    if let Ok(response) = client
        .get("http://localhost:11434/api/ps")
        .send()
    {
        if response.status().is_success() {
            if let Ok(running_models) = response.json::<RunningModelsResponse>() {
                if !running_models.models.is_empty() {
                    return Ok(running_models.models[0].name.clone());
                }
            }
        }
    }

    // If no models are running, check available models and load the first one
    let response = client
        .get("http://localhost:11434/api/tags")
        .send()
        .context("Failed to get available models from Ollama")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to query Ollama models");
    }

    let available_models: AvailableModelsResponse = response
        .json()
        .context("Failed to parse available models response")?;

    if available_models.models.is_empty() {
        anyhow::bail!(
            "No Ollama models are installed. Please install a model first:\n\n\
            For example:\n\
            - ollama pull llama3.2\n\
            - ollama pull llama3.2-vision\n\
            - ollama pull mistral\n\n\
            Visit https://ollama.ai/library for more models"
        );
    }

    // Return the first available model
    Ok(available_models.models[0].name.clone())
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
