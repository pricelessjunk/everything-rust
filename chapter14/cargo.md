# Profiles

- `cargo build` builds in `dev` profile. 
- `cargo build --release` builds in `release` profile. 
- There are different optimization levels. Can be set as follows for different profiles -

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

# Documentation

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

`cargo test` can run the code in the doc as test


# Crate documentations 

```rust
//! # Art
//!
//! A library for modeling artistic concepts.
```

# Re-export inner components

```rust
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```

This can make the inner components exportable as 

```rust
use art::PrimaryColor;
pub art::SecondaryColor;
pub art::mix;
```


