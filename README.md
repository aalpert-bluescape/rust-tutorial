# Rust Tutorial

Requires the rust toolchain installation, see https://doc.rust-lang.org/book/ch01-01-installation.html#installation for details.

This tutorial builds up a noughts and crosses game over multiple iterations as an introduction to rust

# nac 0

The most simple starting point, this is effectively a hello world program introducing students to `cargo run` and `println!`

- `cargo init`
- `cargo run`

# nac 1

Introducing structs, traits, and attributes.

At this stage, we start to prepare code that will be used in a noughts and crosses game.

# nac 2

Introducing memory management and unit tests.

At this stage, we will start to use the code in unit tests.

# nac 3

Playable game.

At this stage, a GUI is added using the slint-ui frame work. This section is largely based off the Slint tutorial: https://slint-ui.com/releases/0.3.1/docs/tutorial/rust/introduction.html

- Add `slint = "0.3.2"` under `[dependencies]` in `Cargo.toml`.
