# `pest_duckyscript`

[![Version](https://img.shields.io/crates/v/pest_duckyscript?style=flat-square)](https://crates.io/crates/pest_duckyscript)
[![Build](https://img.shields.io/travis/Nate-Wilkins/pest_duckyscript/main?style=flat-square)](https://app.travis-ci.com/github/Nate-Wilkins/pest_duckyscript)
[![Downloads](https://img.shields.io/crates/d/pest_duckyscript?color=%230E0&style=flat-square)](https://crates.io/crates/pest_duckyscript)
[![Open Issues](https://img.shields.io/github/issues-raw/Nate-Wilkins/pest_duckyscript?style=flat-square)](https://github.com/Nate-Wilkins/pest_duckyscript/issues)
[![Test Coverage](https://img.shields.io/badge/test%20coverage-96%25-green?style=flat-square)](https://travis-ci.com/Nate-Wilkins/pest_duckyscript)
[![License](https://img.shields.io/github/license/Nate-Wilkins/pest_duckyscript?color=%2308F&style=flat-square)](https://github.com/Nate-Wilkins/pest_duckyscript/blob/main/LICENSE)

> Hak5 DuckyScript and MallardScript language grammers - used for parsing/compilation.

## Install

```
cargo install pest_duckyscript
```

## What does it do?

This package contains [Parsing Expression Grammar (PEG)](https://en.wikipedia.org/wiki/Parsing_expression_grammar) files
for both DuckyScript and MallardScript.

### DuckyScript

See the [Official Documentation](https://docs.hak5.org/hak5-usb-rubber-ducky/duckyscript-tm-quick-reference) for what
DuckyScript looks like or look at the grammar files in this repository.

### MallardScript

Currently a subset of DuckyScript designed to add the following commands:

- `IMPORT relative_path_to.mallardscript`: Which will be inlined when built with
  [mallardscript](https://github.com/Nate-Wilkins/mallardscript).

## Development

Written in rust. You can build and install with the following:

```
cargo build
```

## Roadmap:

- Support for DuckyPad syntax.
