# HTMLER CLI

This is a markdown to html converter. It is written in rust and it is my attempt to learn parsing and lexing.

## Usage

```bash
# Convert a markdown file to html
htmler <path-to-markdown-file>
```

Note: Only the markdown syntax used in [test.md](/test.md) is supported.
More features will be added in the future.

## Building

```bash
# Clone the repo
git clone https://github.com/newtoallofthis123/htmler_cli.git
# Change directory
cd htmler_cli
# Build
cargo build --release
# Run
./target/release/htmler <path-to-markdown-file>
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.