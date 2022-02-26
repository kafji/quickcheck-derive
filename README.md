# Quickcheck Derive

Procedural macro for deriving Quickcheck's Arbitrary trait.

## Usage

```rust
use quickcheck_derive::Arbitrary;

#[derive(Arbitrary, Clone)]
struct Person {
    name: String,

    #[arbitrary(generator = "gen_age")]
    age: u8,
}

fn gen_age(g: &mut quickcheck::Gen) -> u8 {
    use quickcheck::Arbitrary;
    loop {
        let age = u8::arbitrary(g);
        if age > 20 {
            break age;
        }
    }
}
```

## Install

```toml
quickcheck-derive = { git = "https://github.com/kafji/quickcheck-derive", tag = "v0.2.0" }
```
