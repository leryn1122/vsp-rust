# README

## TL;DR

A toy language compiler during learning custom compiler and Rust lang.

## Table of Contents

- [Background](#background)
- [RoadMap](#RoadMap)
- [Install](#install)
- [Usage](#usage)
- [Related Efforts](#related-efforts)
- [Maintainers](#maintainers)
- [License](#license)

## Background

A toy language compiler during learning custom compiler and Rust lang.

## RoadMap

Long term:

| Command   | Intro                              |
| --------- | ---------------------------------- |
| `vspc`    | Compiler                           |
| `vspr`    | Runtime                            |
| `vsps`    | Process tool                       |
| `vspstk`  | Stack trace tool                   |
| `vspx`    | Tool of Compression and Extraction |

## Install

This project uses [rust](https://www.rust-lang.org/) and [cargo](https://npmjs.com). Go check them out if you don't have them locally installed.

```bash
git clone https://github.com/leryn1122/vsp.git
cd vsp/

cargo build --release
```

## Usage

```plaintext
vspc <source> [ options [ params ... ] ... ]
```

```bash
vspc test.vsp

vspc test.vsp --verbose

vspc --version

vspc --help
```

### 

Lexer:  String => Path => File => TokenStream
Parser: TokenStream => AstNode
Syntax: AstNode =>

## Related Efforts

Those repos are referenced on:

- [Xie-Jason/GloomScript](https://github.com/Xie-Jason/GloomScript)
- [douchuan/jvm](https://github.com/douchuan/jvm)
- [rhaiscript/rhai](https://github1s.com/rhaiscript/rhai)

## Maintainers

[@Leryn](https://github.com/leryn1122).

## License

[MIT](LICENSE) © Leryn