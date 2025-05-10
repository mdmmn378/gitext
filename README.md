# gitext

A command-line tool that converts a Git repository into a single text file optimized for LLM consumption.

## Author

[Mamun](https://github.com/mdmmn378)

## Features

- Extracts source code and documentation from Git repositories
- Supports multiple file types (Rust, Python, Go, TypeScript, JavaScript, Java, C++, C, Markdown, etc.)
- Respects .gitignore by default
- Configurable output file
- Simple and fast

## Installation

```bash
cargo install --path .
```

## Usage

```bash
gitext --repo /path/to/repo --output summary.txt
```

### Options

- `-r, --repo`: Path to Git repository (required)
- `-o, --output`: Output file path (default: output.txt)
- `--no-gitignore`: Include files ignored by .gitignore

## Example

```bash
gitext -r ./my-project -o project-summary.txt
```

## License

MIT
