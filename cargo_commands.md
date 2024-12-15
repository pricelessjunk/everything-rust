# Create new Cargo project/sub-project

```
mkdir <projectname>
cargo init <projectname>
cargo new <projectname>
```

# Create run specific sub-project

```
cargo run --bin <projectname>
```

# Test sub-project lib

```
cargo test --lib chapter12
```

# Create new lib

```
cargo new <name> --lib
```

- Start point for libraries are `src/lib.rs` instead of `src/main.rs`
