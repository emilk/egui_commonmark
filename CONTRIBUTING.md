## How to contribute

First of all thank you for considering it. Check out
[ARCHITECTURE.md](ARCHITECTURE.md) for an overview of how the repo is put together.

### Running tests

You won't be able to run the tests without enabling the `macros` feature
as one of the examples depend on it.

`cargo test --features macros`

### Snapshot tests

Rendering is checked with [egui_kittest](https://github.com/emilk/egui/tree/master/crates/egui_kittest#snapshot-testing)
snapshot tests in `egui_commonmark/tests/`. When you change how something renders,
update the snapshots and commit the new PNG files:

`UPDATE_SNAPSHOTS=1 cargo test --features macros`


### Debugging the proc macros

To see the output of the macros enable the `dump-macro` feature.
For the macro example the output can be viewed like this:

```sh
cargo r --example macro --features dump-macro,macros -- dark | rustfmt --edition=2024 | less
```
