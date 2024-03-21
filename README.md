# oats

![Used 'nightly' toolchain](https://img.shields.io/badge/toolchain-nightly-important)
![Version 0.2.1](https://img.shields.io/badge/version-0.2.1-informational)

**Short, unique IDs without the hassle of random UUIDs in a multi-threaded enviroment.**

The library was created to simplify the use of UUIDs in a multi-server environment. It was inspired by [BinCheng's snowflake-rs](https://github.com/BinChengZhao/snowflake-rs) library, but is designed to be simpler and only allows for up to 256 independent nodes. The library includes only the node ID, a 12-bit sequence number, and the duration since the provided timestamp (44 bits).

This library also includes built-in support for multithreading, enabling the creation of a single WrappedBowl instance that can be used concurrently in multiple instances. Just call `WrappedBowl::generate()` to obtain a unique ID that is distinct from all other generated IDs in the world. The ID consists of 9 bytes: 1 byte for the node ID and 8 bytes for the local unique identifier. A string representation can also be rendered, which is up to 14 characters long.

When using the Unix timestamp in milliseconds, the theoretical limit is Mon Jun 23 2527 06:20:44 UTC+0000 (Coordinated Universal Time). This should be sufficient for any long-running service.

## Thanks to

- [BinCheng's snowflake-rs](https://github.com/BinChengZhao/snowflake-rs) which was the inspiration for this project.
- [John Vandenberg's comment](https://github.com/Skailys/oats-rs/issues/1) on naming conflict what was the cause that I uploaded this library to [crates.io](https://crates.io/crates/oats-rs).

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

    /* To use this instance now in another thread just clone it */
    let cloned_bowl = wrapped_bowl.clone();
    handles.push(thread::spawn(move || {
        cloned_bowl.generate();
    }));
    
    assert_eq!(oat.node(), 1);
```

### Syntax of Oats

When using ToString, the Oat object is displayed in a mixed format that includes the node ID and a local unique identifier (LUID) with a timestamp and sequence ID. The LUID is encoded as a URL-safe base64 string without padding.

```rust
    const ENGINE: FastPortable = FastPortable::from(&URL_SAFE, NO_PAD);
    format!("{:X>2X}{}", &self.node, encode_engine(&self luid.to_le_bytes(), &ENGINE))
```
