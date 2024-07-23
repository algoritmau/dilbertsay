# dilbertsay

A Rust library for printing out text with Dilbert characters as mascots.

## Features

- Takes a string as a positional argument
- Takes a `-a`/`--alt` flag that makes Dilbert appearance different.
- Takes a `-c`/`--character` optional value to select a particular character.
- Takes a `-h`/`--help` flag to print a help message.
- Takes a `-v`/`--version` flag to print version information.
- Prints the image in color.
- Error handling: prints any error message to STDERR.
- Piping: accepts STDIN as input and allows the output to be piped to other programs.
- Run integration tests.

## Build Requirements

Only a stable version of the Rust compiler is needed.

## How to use Ricksay

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
dilbertsay = "0.0.1"
```

or run:

```bash
cargo add dilbertsay
```

## License

TBD

