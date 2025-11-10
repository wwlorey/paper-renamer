# Paper Renamer - Vision Document

## Overview

A terminal application that automatically renames academic paper PDFs using a standardized naming convention based on the paper's metadata (first author, year, and title).

## Target Naming Convention

```
<first-author-last-name><year><paper-title>.pdf
```

**Rules:**
- All lowercase
- Dashes (`-`) instead of spaces
- No special characters

**Example:**
```
Original: article.pdf
Renamed:  james-2002-sample-research-paper.pdf
```

## User Experience

### Command Usage
```bash
rename article.pdf
```

### Interaction Flow
1. User runs the command with a PDF file path
2. Application reads and analyzes the PDF
3. Application extracts metadata using a local LLM:
   - First author's last name
   - Publication year
   - Paper title
4. Application generates a proposed filename
5. Application prompts user: `Would you like to rename article.pdf to james-2002-sample-research-paper.pdf? (y/n/edit)`
6. User options:
   - `y` (yes): Rename the file immediately
   - `n` (no): Cancel the operation
   - `e` (edit): Open an editor to modify the suggested filename
7. File is renamed (or operation is cancelled)

## Technical Architecture

### Core Components

1. **PDF Parser**
   - Extract text content from PDF
   - Focus on first page/header for metadata

2. **LLM Integration**
   - Use a local LLM (e.g., Ollama, llama.cpp)
   - Send extracted text with prompt to identify:
     - First author's last name
     - Year of publication
     - Full paper title

3. **Filename Generator**
   - Format extracted metadata into standardized format
   - Sanitize: lowercase conversion, space-to-dash, remove special chars

4. **Interactive CLI**
   - Present suggested filename
   - Handle user input (yes/no/edit)
   - Perform file renaming operation

### Technology Stack (Proposed)

- **Language**: Rust
  - Excellent performance for PDF processing
  - Memory safety without garbage collection
  - Single binary distribution (no runtime dependencies)
  - Strong type system and error handling
  - Easy cross-platform compilation

- **Key Libraries (Crates)**:
  - `pdf-extract` or `lopdf`: PDF text extraction
  - `ollama-rs` or `reqwest`: Ollama API integration
  - `clap`: CLI framework and argument parsing
  - `dialoguer` or `inquire`: Interactive prompts
  - `serde` and `serde_json`: JSON serialization for LLM API
  - `anyhow` or `thiserror`: Error handling
  - `tokio`: Async runtime for API calls

### LLM Options

1. **Ollama** (Recommended)
   - Easy to install and use
   - Supports various models (llama, mistral, phi, etc.)
   - Simple HTTP REST API (easy to integrate with Rust)
   - Model runs locally
   - Good performance and model selection

2. **llama.cpp with HTTP server**
   - Lightweight C++ implementation
   - Can run as HTTP server
   - Excellent performance
   - Direct Rust bindings available via `llama-cpp-rs`

3. **Direct HTTP API**
   - Any local LLM with HTTP interface
   - Use `reqwest` crate for API calls
   - Maximum flexibility

## Implementation Phases

### Phase 1: MVP
- Basic PDF text extraction (first page)
- Integration with Ollama for metadata extraction
- Simple yes/no confirmation
- File renaming functionality

### Phase 2: Enhanced UX
- Interactive filename editing
- Preview of filename before confirmation
- Better error handling
- Support for batch processing

### Phase 3: Advanced Features
- Multiple PDF format support
- Caching of processed papers
- Configuration file for naming preferences
- Support for different citation styles
- Handle edge cases (no author, multiple authors, etc.)

## User Stories

1. **As a researcher**, I want to quickly rename downloaded papers so that my file system is organized.

2. **As a graduate student**, I want to ensure consistent naming across my paper library without manual work.

3. **As a librarian**, I want to batch-process papers to maintain a standardized digital archive.

## Success Criteria

- Successfully extracts metadata from 90%+ of academic papers
- Generates accurate filenames without manual editing in 80%+ of cases
- Runs in under 10 seconds per paper (including LLM inference)
- Intuitive CLI that requires minimal learning

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| LLM extracts incorrect metadata | Allow user to edit before confirming |
| PDF has no extractable text (scanned images) | Add OCR capability or graceful failure message |
| LLM inference is too slow | Use smaller, faster models; optimize prompts |
| Paper has no clear author/year | Provide fallback naming or request manual input |

## Future Considerations

- GUI version for non-technical users
- Cloud LLM option for users without local GPU
- Integration with reference managers (Zotero, Mendeley)
- DOI-based metadata extraction as fallback
- Browser extension for direct downloads

## Configuration Example

```yaml
# ~/.paper-renamer/config.yaml
llm:
  provider: ollama
  model: llama3.2

naming:
  format: "{author}{year}{title}"
  separator: "-"
  case: lowercase
  max_title_words: 10

behavior:
  auto_confirm: false
  backup_original: true
  verbose: false
```

## Example Outputs

### Successful Rename
```
$ rename "Attention Is All You Need.pdf"

Analyzing PDF...
Extracted metadata:
  - First Author: Vaswani
  - Year: 2017
  - Title: Attention Is All You Need

Proposed filename: vaswani-2017-attention-is-all-you-need.pdf

Would you like to rename this file? (y/n/e): y

✓ File renamed successfully!
```

### Edit Mode
```
$ rename smith_paper.pdf

Analyzing PDF...

Proposed filename: smith-2020-deep-learning-for-natural-language-processing.pdf

Would you like to rename this file? (y/n/e): e

Edit filename: smith-2020-deep-learning-nlp.pdf

✓ File renamed successfully!
```

### Error Case
```
$ rename scanned_paper.pdf

Analyzing PDF...
⚠ Warning: Could not extract text from PDF. The file may be a scanned image.

Would you like to enter metadata manually? (y/n): n

Operation cancelled.
```
