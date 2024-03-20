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

### Syntax of oats

When using ToString, the Oat object is displayed in a mixed format that includes the node ID and a local unique identifier (LUID) with a timestamp and sequence ID. The LUID is encoded as a URL-safe base64 string without padding.

```rust
    const ENGINE: FastPortable = FastPortable::from(&URL_SAFE, NO_PAD);
    format!("{:X>2X}{}", &self.node, encode_engine(&self luid.to_le_bytes(), &ENGINE))
```
