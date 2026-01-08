# Minigrep

Command line tool to search for a string within a file - outputs line containing string

## Usage

```bash
git clone git@github.com:brysonbw/minigrep.git
```

```bash
cd minigrep
```

## Run

| Command                                      | Description                   | Example                          |
| -------------------------------------------- | ----------------------------- | -------------------------------- |
| `cargo run -- <query_string> <file_path>`    | Search for a string in a file | `cargo run -- hello text.txt`    |
| `cargo run -- <query_string> <file_path> -i` | Case-insensitive search       | `cargo run -- hello text.txt -i` |

## Options

| Flag           | Description              |
| -------------- | ------------------------ |
| `-i`           | Ignore case              |
| `-h`, `--help` | Display help information |

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

## Format

```bash
cargo fmt
```
