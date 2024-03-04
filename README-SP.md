# 󱥙

_[󱤪󱦖󱥁󱤙󱥬󱦐󱤌󱤿󱤡󱤎󱦑󱤬󱦗󱥁󱦘](./README.md)_

󱤎󱥩󱤃󱥂󱥍󱦗󱥬󱦖󱥔

![󱥠](.github/example.png)

## 󱤓

󱤙 `cargo`: `cargo install --git https://github.com/Brian3647/seme`

## 󱥬

```sh
Usage: seme [OPTIONS] <WORD>

Arguments:
  <WORD>  The word to get the definition of

Options:
  -j, --json         Show the RAW JSON response from the API
  -t, --toki <TOKI>  The language used to get the word definitions. [default: en]
  -h, --help         Print help
```

```sh
# Examples

seme jan
seme toki --json # raw json data
seme nanpa --toki es # give the definition in spanish
```
