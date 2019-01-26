# Changelog

## v0.2.1

Released Jan 26, 2019

### Added

- Add [XLS](https://en.wikipedia.org/wiki/Microsoft_Excel_file_format) to [TSV](https://en.wikipedia.org/wiki/Tab-separated_values) converter: `xls2tsv`
- Add `dictionary.starts_with` and `map.range` methods.

### Fixed

- Fix an error included column names in the words.
- Fix [Open Korean Dictionary](https://opendict.korean.go.kr/main) crawler’s [escape sequence](https://en.wikipedia.org/wiki/Escape_sequence) bug.
- Remove `extern crate`; it is going away in the 2018 edition.

## v0.2.0

Released Jan 23, 2019

### Added

- Add `Words` type.
- Add `crawler` module.
  - [Open Korean Dictionary](https://opendict.korean.go.kr/main) crawler (not recommended)
  - [Standard Korean Dictionary](http://stdweb2.korean.go.kr/main.jsp) crawler
- Add `fs` module.
  - [TSV](https://en.wikipedia.org/wiki/Tab-separated_values) reader / writer
- Add `parser` module.
  - [Open Korean Dictionary](https://opendict.korean.go.kr/main) [XLS](https://en.wikipedia.org/wiki/Microsoft_Excel_file_format) parser
- Add `trie` module.

### Changed

- Change `DictionaryItem` struct name to `Word`.
  - Change `word` field name to `entry`.
- Now `dictionary.find` returns `Option<&Words>`.

### Removed

- Remove `dictionary.create_from_web` method.
- Remove `dictionary.create_from_file` and `dictionary.save_as_tsv` method.
- Remove `dictionary.find_all` method.

## v0.1.3

Released Nov 29, 2018

### Fixed

- Fix a bug that `\n` exists in the meaning of a dictionary items.

## v0.1.2

Released Nov 21, 2018

### Fixed

- Fix a bug that missed some words.

## v0.1.1

Released Nov 20, 2018

### Fixed

- Fix `dictionary.save_as_tsv(path)` method's arguments: `self` -> `&self`

## v0.1.0

Released Nov 18, 2018

First Release
