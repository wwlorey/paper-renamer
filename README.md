# Paper Renamer

A Rust-based terminal application that automatically renames academic paper PDFs using LLM-extracted metadata. The application analyzes PDF content and generates standardized filenames based on the paper's first author, publication year, and title.

## Features

- Extracts text from PDF files automatically
- Uses local LLM (Ollama) to identify paper metadata
- Generates standardized filenames: `<author>-<year>-<title>.pdf`
- Interactive CLI with confirmation and editing options
- No external API costs - runs completely locally
- Fast and memory-efficient Rust implementation

## Installation

### Prerequisites

1. **Rust** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Ollama** - Local LLM runtime
   ```bash
   # macOS/Linux
   curl -fsSL https://ollama.com/install.sh | sh

   # Start Ollama service
   ollama serve
   ```

3. **Download an LLM model** (in a separate terminal)
   ```bash
   ollama pull llama3.2
   ```

### Build from source

```bash
git clone https://github.com/wwlorey/paper-renamer.git
cd paper-renamer
cargo build --release
```

The binary will be available at `target/release/paper-renamer`.

## Usage

### Basic usage

```bash
paper-renamer path/to/paper.pdf
```

The application will automatically detect and use any available Ollama model.

### Example interaction

```
$ paper-renamer "Attention Is All You Need.pdf"

Detecting available Ollama model...
Using model: llama3.2

Analyzing PDF...
Extracting metadata using LLM...

Extracted metadata:
  - First Author: Vaswani
  - Year: 2017
  - Title: Attention Is All You Need

Proposed filename: vaswani-2017-attention-is-all-you-need.pdf

Would you like to rename 'Attention Is All You Need.pdf' to 'vaswani-2017-attention-is-all-you-need.pdf'?
  > Yes - rename the file
    No - cancel
    Edit - modify filename

✓ File renamed successfully!
  Attention Is All You Need.pdf -> vaswani-2017-attention-is-all-you-need.pdf
```

## Naming Convention

The application follows this standardized naming convention:

**Format:** `<first-author-last-name>-<year>-<paper-title>.pdf`

**Rules:**
- All lowercase
- Dashes (`-`) instead of spaces
- No special characters
- Only alphanumeric characters and dashes

**Examples:**
- `vaswani-2017-attention-is-all-you-need.pdf`
- `lecun-1998-gradient-based-learning-applied-to-document-recognition.pdf`
- `goodfellow-2014-generative-adversarial-networks.pdf`

## Configuration

### Supported Ollama models

The application automatically detects and uses any available Ollama model. It works with any model that supports JSON output. Recommended models:
- `llama3.2` (fast and accurate)
- `llama3.1`
- `mistral`
- `phi3`

The application will:
1. First check for any models currently running
2. If no models are running, it will use any installed model
3. If no models are installed, it will provide clear instructions on how to install one

### Command-line options

```
Options:
  -h, --help           Print help
```

## Troubleshooting

### "Cannot connect to Ollama"

Make sure Ollama is running:
```bash
ollama serve
```

If Ollama is not installed, visit https://ollama.ai to download it.

### "No Ollama models are installed"

Install a model using:
```bash
ollama pull llama3.2
```

For more models, visit https://ollama.ai/library

### "No text could be extracted from the PDF"

The PDF may be a scanned image. Future versions will support OCR for scanned documents.

### LLM returns incorrect metadata

Use the Edit option to manually correct the filename. The application automatically selects an available model, but you may want to try different models by pulling them with `ollama pull <model-name>`.

## Development

### Running tests

```bash
cargo test
```

### Building for release

```bash
cargo build --release
```

### Project structure

```
src/
├── main.rs      - CLI argument parsing and main application flow
├── pdf.rs       - PDF text extraction
├── llm.rs       - Ollama LLM integration and metadata extraction
├── filename.rs  - Filename generation and sanitization
├── ui.rs        - Interactive CLI prompts and user feedback
└── renamer.rs   - File renaming operations
```

## Roadmap

- [ ] Batch processing support
- [ ] Configuration file for naming preferences
- [ ] OCR support for scanned PDFs
- [ ] Manual metadata entry fallback
- [ ] DOI-based metadata extraction
- [ ] Multiple citation format support

## License

See LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
