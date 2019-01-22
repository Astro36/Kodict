# Kodict

> Korean Dictionary Manager for Rust

[![Crates.io](https://img.shields.io/crates/v/kodict.svg?style=for-the-badge)](https://crates.io/crates/kodict) [![Crates.io](https://img.shields.io/crates/d/kodict.svg?style=for-the-badge)](https://crates.io/crates/kodict)

## Changelog

See [CHANGELOG](./CHANGELOG.md)

## Features

- Crawl words from the online dictionary.
- Find the given word from a dictionary.
- Read and write a dictionary file.

### Supported online dictionaries available crawling

- [Open Korean Dictionary(우리말샘)](https://opendict.korean.go.kr/main)
- [Standard Korean Dictionary(국립국어원 표준국어대사전)](http://stdweb2.korean.go.kr/main.jsp)

Crawl [Standard Korean Dictionary](http://stdweb2.korean.go.kr/main.jsp) and save as [TSV](https://en.wikipedia.org/wiki/Tab-separated_values) file:

```rust
extern crate kodict;

use kodict::{crawler, fs};
use std::path::Path;

fn main() {
    let words = crawler::get_standard_dictionary_words();
    fs::write_as_tsv(Path::new("./dictionary.tsv"), &words);
}
```

**Notice: Crawling [Open Korean Dictionary](https://opendict.korean.go.kr/main) spends too much time. You can download it [here](https://opendict.korean.go.kr/member/memberDownloadList).**

Parse [Open Korean Dictionary](https://opendict.korean.go.kr/main) `words.xls`:

```rust
extern crate kodict;

use kodict::{Dictionary, parser};
use std::path::Path;

fn main() {
    let words = parser::parse_open_dictionary_xls(Path::new("./words.xls"));
    let dictionary = Dictionary::new(words);
}
```

## Installation

- Install with [cargo](https://crates.io/crates/kodict).

- Clone the repo:

```bash
git clone https://github.com/Astro36/kodict.git
```

## Usage

### API Documentation

See [API](https://docs.rs/kodict)

## License

```text
Copyright (c) 2019 Seungjae Park

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

Kodict is licensed under the [MIT License](./LICENSE).
