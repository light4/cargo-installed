# cargo-installed

Easy manage programs installed by `cargo install`.

[![CI](https://github.com/light4/cargo-installed/actions/workflows/test.yaml/badge.svg)](https://github.com/light4/cargo-installed/actions/workflows/test.yaml)
[![build-and-release](https://github.com/light4/cargo-installed/actions/workflows/build-and-release.yaml/badge.svg)](https://github.com/light4/cargo-installed/actions/workflows/build-and-release.yaml)

## Install

```bash
# from crates.io
cargo install cargo-installed --force
# from git repo
cargo install --git https://github.com/light4/cargo-installed.git --force
```

## Usage

```bash
~ on  master 🕙 22:28:47
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
