# m4b-extractor

[![rust][rust-version-src]][rust-version-href]
[![tests][tests-src]][tests-href]

[Rust](https://www.rust-lang.org/) CLI tool to extract chapters, metadata and cover for M4B Audiobook. Based on idea of [Hasan Arous](https://unix.stackexchange.com/questions/499179/using-ffmpeg-to-split-an-audible-audio-book-into-chapters).

Available on [crates.io](https://crates.io/crates/m4b-extractor).

## Requirements

- [Rust](https://www.rust-lang.org/)
- [FFmpeg](https://ffmpeg.org/) installed and available in your `PATH`
- [jq](https://jqlang.org/) installed and available in your `PATH`
- [yq](https://github.com/mikefarah/yq) installed and available in your `PATH`

## Installation

This fork of `m4b-extractor` is not currently packaged anywhere. To install it you will have to compile it and optionally add it to your operating system's `PATH` variable. Automated builds will be available in the future.

## Usage

You have to pass the path to the `.m4b` file you want to extract chapters from.

```bash
m4b-extractor /path/to/input.m4b
```

You will get a directory named `<input_file>_chapters` containing:

- Each chapter as a separate `.mp3` file, or `flac` file if `-f flac` is specified.
- A `metadata.json` file with the metadata of the book.
- A `tags.yaml` file with the tags of the book.
- A `folder.jpg` file with the cover of the book.

```plain
1_Chapter 01.mp3
2_Chapter 02.mp3
3_Chapter 03.mp3
# ...
folder.jpg
metadata.json
tags.yaml
```

### Options

```bash
m4b-extractor --help
```

- `-o`, `--output <OUTPUT>`: Specify the output directory for extracted chapters (default: `<input_file>_chapters`).
- `-k`, `--keep`: Keep the original `.m4b` files without converting them to `.mp3`.
- `-k`, `--conversion-format`: Which format to convert the m4b file(s) to, FLAC or MP3.
- `-q`, `--quality <QUALITY>`: Specify the conversion quality (1=best, 9=worst) for `.mp3` files (default: `2`).
- `-s`, `--sanitize`: Sanitize filenames by replacing invalid characters with underscores (default: `false`).
- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print the version of the tool.

## Test and publish

Build and test the package:

```bash
# debug build
cargo build
# release build
cargo build --release
cargo test
```

Test publishing:

```bash
cargo publish --dry-run
```

Publish the package to [crates.io](https://crates.io):

```bash
cargo publish
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

[rust-version-src]: https://img.shields.io/badge/Rust-v1.88.0-000000?colorA=18181B&logo=Rust&logoColor=ffffff
[rust-version-href]: https://www.rust-lang.org/
[tests-src]: https://img.shields.io/github/actions/workflow/status/ewilan-riviere/m4b-extractor/run-tests.yml?branch=main&label=tests&style=flat&colorA=18181B
[tests-href]: https://github.com/ewilan-riviere/m4b-extractor/actions
