# posix-shell-rust

A POSIX-compliant shell implementation written in Rust, built from the ground up
against the [POSIX.1-2017 shell grammar](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html).

## Status

Work in progress. The following are implemented:

- Lexer with full operator and reserved word recognition
- Recursive descent parser covering the complete POSIX shell grammar
- AST pretty-printer

## Quick start

```sh
cargo build
./target/debug/posix-shell-rust -p -c 'echo hello world'
./target/debug/posix-shell-rust --pretty-printer -c 'echo hello world' # string
./target/debug/posix-shell-rust -p file.txt # file 
./target/debug/posix-shell-rust -p # standard input
```

It is recommended to use single quotes when using string as input so your own
shell won't interact with the input.

### Flags

| Flag | Description |
|------|-------------|
| `-c <string>` | Execute a command string |
| `-p` or `--pretty-print` | Print the parsed AST instead of executing |

## Project structure

src/ 
├── ast/ # AST definitions 
│ └── pretty_print/ # AST pretty-printer 
├── lexer/ # Tokenization 
├── parser/ # Recursive descent parser 
├── exec/ # Execution engine (WIP) 
│ └── builtin/ # Built-in commands 
├── io_backend/ # Input handling (stdin, files, etc.) 
├── variables/ # Shell variables & environment 
└── main.rs # Entry point

## License

MIT — see [LICENSE](LICENSE).
