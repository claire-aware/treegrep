# treegrep

Grep utility for outlines, trees, and when indentation matters.

Useful for working with TODO lists or any indented file format where you need the full context (ancestors) of a match.

## Install

Requires [Rust](https://rustup.rs). Then:

```sh
cargo install --path .
```

Or just build:

```sh
cargo build --release
```

## Usage

Usage is similar to grep. Works with one or more files, or piped from stdin.

```
treegrep PATTERN [FILE]...
```

### Example

```sh
treegrep "\[>\]" ~/.tasks
```

Output:

```
[ ] some group of tasks
    [ ] subtasks
        [>] current task here!
```

When multiple files are given, matches are prefixed with the filename and line number:

```
tasks.txt(3): [>] current task here!
```

## Ideas / Roadmap

- Configurable tab width (currently hardcoded to 4)
- `--leaves` / `--parents` flags to control what gets printed
- Directory tree mode (`-d`) — treat a directory as input rather than files
- Pipe support from `tree` output (treating tree's branch characters as whitespace)

Pull requests and ideas welcome.

## Contributors

- Grady O'Connell ([@flipcoder](https://github.com/flipcoder))

Thanks to #rust and /r/rust for help with bugs and tips!

## License

MIT License — Copyright (c) 2013 Grady O'Connell

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
