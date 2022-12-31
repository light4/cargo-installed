# cargo-installed

Easy manage programs installed by `cargo install`.

[![CI](https://github.com/light4/cargo-installed/actions/workflows/test.yaml/badge.svg)](https://github.com/light4/cargo-installed/actions/workflows/test.yaml)
[![build-and-release](https://github.com/light4/cargo-installed/actions/workflows/build-and-release.yaml/badge.svg)](https://github.com/light4/cargo-installed/actions/workflows/build-and-release.yaml)

## Install

```bash
cargo install --git https://github.com/light4/cargo-installed.git --force
```

## Usage

```bash
~ on  master via 🐍 v3.10.2 🕙 12:59:50
❯ cargo installed --help
Easy manage programs installed by `cargo install`

Usage: cargo installed [OPTIONS]

Options:
  -u, --upgrade          upgrade all outdated
  -o, --outdated         show outdated
  -l, --ignore-local     ignore installed from local space, enabled by default
  -i, --ignore <IGNORE>  ignore from upgrade
  -v, --verbose          show details
  -h, --help             Print help information
```
