# CLI Rust based application

The aim of this project is to create a fully random CLI Password Generator.


## Instalation

```bash
git clone https://github.com/paugarcia32/CLI-Password-Generator.git
```

## Available options

- Length: `-l` or `--length`
- Lowercase: `-w` or `--lowercase`
- Uppercase: `-u` or `--uppercase`
- Numbers: `-n` or `--numbers`
- Special Characters: `-s` or `--special`

> [!note]
> There is a input validation, inputs that are not between 5 and 40 characters are not available!


## How to run the appication

```bash
cargo r -- -l 12 -w -u -n -s
```
