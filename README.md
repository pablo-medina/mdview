# MDView - Markdown Viewer

A cross-platform Markdown viewer application built with Rust, compatible with Windows, Linux, and macOS.

## Features

- Modern graphical user interface built with egui
- Complete support for standard Markdown syntax
- Extended Markdown features:
  - Tables
  - Task lists
  - Strikethrough text
  - Footnotes
  - Smart punctuation
- Command-line interface support
- Drag and drop file support
- Raw markdown source view
- Native file dialogs
- Persistent settings and themes
- Cross-platform compatibility

## Installation

### Prerequisites

Rust must be installed on your system. If not already installed, it can be obtained from [rustup.rs](https://rustup.rs/).

### Building from source

1. Clone or download this repository
2. Navigate to the project directory:
   ```bash
   cd mdview
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The executable will be generated at `target/release/mdview.exe` (Windows) or `target/release/mdview` (Linux/macOS)

## Usage

### Command Line Interface

#### Basic usage
```bash
# Open the application without a file
mdview

# Open a specific markdown file
mdview document.md
mdview /path/to/file.markdown

# Display help
mdview --help

# Show version information
mdview --version
```

#### Running from source
```bash
# Development mode
cargo run

# Development mode with file
cargo run -- document.md

# Release mode
./target/release/mdview document.md
```

### Opening Files

The application supports multiple methods for opening markdown files:

1. **File Menu**: Use "File > Open..." from the menu bar
2. **Toolbar Button**: Click the folder icon in the toolbar
3. **Drag and Drop**: Drag markdown files directly onto the application window
4. **Command Line**: Pass the file path as an argument when launching

### View Options

- **Raw View Toggle**: Switch between rendered and raw markdown source using the code icon in the toolbar
- **Settings Panel**: Access theme configuration and application information via the gear icon

### File Association

To set MDView as the default markdown viewer:

1. Right-click on a `.md` file
2. Select "Open with" → "Choose another app"
3. Browse to the MDView executable (`target/release/mdview.exe`)
4. Check "Always use this app to open .md files"

## Supported Formats

- `.md` - Markdown files
- `.markdown` - Markdown files
- `.txt` - Plain text files

## Key Dependencies

- `eframe` - Application framework for GUI applications
- `egui` - Immediate mode GUI library
- `pulldown-cmark` - Markdown parser implementation
- `rfd` - Native file dialog support
- `clap` - Command-line argument parser
- `egui-phosphor` - Icon library for UI elements

## Development

To contribute to the project:

1. Fork and clone the repository
2. Run `cargo run` to compile and execute in development mode
3. Run `cargo test` to execute the test suite
4. Submit pull requests with your improvements

## Architecture

The application follows a modular design:

- **Main Application**: Built with eframe/egui for cross-platform GUI
- **Markdown Parsing**: Uses pulldown-cmark for robust markdown processing
- **Settings Management**: Persistent configuration using serde
- **File Handling**: Native file operations with cross-platform compatibility

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Author

Created by Pablo Medina (2025)