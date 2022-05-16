<h1 align="center">Chorus</h1>

<p align="center">Cargo for COBOL</p>

[![crates.io](https://img.shields.io/crates/v/chorus.svg)](https://crates.io/crates/chorus)
[![github.com](https://github.com/Fuwn/chorus/actions/workflows/check.yaml/badge.svg?branch=main)](https://github.com/Fuwn/chorus/actions/workflows/check.yaml)

Chorus is a Cargo-like utility for building and distributing COBOL packages.

Chorus is under heavy development and functions, kinda. It is nowhere near
finished, but few features work.

## Installation

```shell
$ cargo install --force --git https://github.com/Fuwn/chorus
```

## Usage

### Creating a new Chorus package

```shell
$ chorus new your_project_name
```

### Building a Chorus package

```shell
$ chorus build --path your_project_name
```

## License

This project is licensed with the
[GNU General Public License v3.0
](https://github.com/Fuwn/rust-crate-template/blob/main/LICENSE).
