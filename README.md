# latex-equivalencer #

## Debug by Evcxr ##

You can debug by [Evcxr Rust REPL](https://github.com/google/evcxr/blob/main/evcxr_repl/README.md).

```console
$ cd /path/to/latex-equivalencer
$ evcxr
Welcome to evcxr. For help, type :help
>> :dep latex_equivalencer = { package = "latex-equivalencer", path = "." }
>> use latex_equivalencer::*
```

## Test ##

```console
$ cargo test --all -- --nocapture
```

## Lint ##

```console
$ cargo clippy --all-targets --all-features -- -D warnings -A clippy::needless_return
```

## Format ##

```console
$ cargo fmt --all -- --check
```
