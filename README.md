# oats

![Used 'nightly' toolchain](https://img.shields.io/badge/toolchain-nightly-important)
![Version 0.1.0](https://img.shields.io/badge/version-0.1.0-informational)

**Short, unique ids without the hassle of random uuids.**

## Thanks

- Inspired from <https://github.com/BinChengZhao/snowflake-rs>.

## Getting started

### Adding library to project

```bash
cargo add oats-rs
```

### Using it to generate unique identifiers

```rust
    use oats::bowl::{GenerationBehavior, WrappedBowl};
    use std::time::SystemTime;
    
    let wrapped_bowl = WrappedBowl::of(1, GenerationBehavior::Normal, Some(SystemTime::now()));
    let oat = wrapped_bowl.generate();
    
    assert_eq!(oat.node(), 1);
```
