# Create new Cargo project/sub-project

```sh
mkdir <projectname>
cargo init <projectname>
cargo new <projectname>
```

# Sub-project

## Create subproject

```sh
cargo new <projectname>
```
- This creates a project under the root project.

## Run subproject

```sh
cargo run --bin <projectname>
cargo run --bin <projectname> -- arg1 arg2
cargo run -p <projectname> -- arg1 arg2
```

## Test sub-project lib

```sh
cargo test
cargo test --lib chapter12
cargo test -p chapter12
```

# Create new lib

```sh
cargo new <name> --lib
```

- Start point for libraries are `src/lib.rs` instead of `src/main.rs`


# Create doc

```sh
cargo doc --open
```

- Created under target/doc

# crates.io

## Logging in

```sh
$ cargo login
```

api token stored in `~/.cargo/credentials`.

## publish

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"
```

```sh
cargo publish
```

- To publish a new version just update the version and publish a new version

## Yanking a version

To prevent new project from accessing a version, say which is broken.
```sh
cargo yank --vers 1.0.1
```

## Undo the yank

```sh
cargo yank --vers 1.0.1 --undo
```
- A yank does not delete anything.

# Cargo workspaces (sub projects)

```toml
[workspace]
resolver = "2"  # This is the cargo resolver algorithm version
members = ["adder"]
```

# Install using cargo

```sh
cargo install <package>
```

# Cargo packages

If a binary in your `$PATH` is named `cargo-something`, you can run it as if it was a Cargo subcommand by running `cargo something`

```sh
cargo --list
```