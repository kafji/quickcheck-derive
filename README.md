# Quickcheck Derive

## Usage

```rust
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, Clone)]
struct Person {
    name: String,
    age: u8,
}
```

## Install

```toml
quickcheck-derive = { git = "https://github.com/kafji/quickcheck-derive", tag = "v0.1.0" }
```
