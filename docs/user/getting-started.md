# Getting Started

## Prerequisites

- Rust toolchain (edition 2021 or later)
- Cargo package manager

## Installation

Add `rudi` to your `Cargo.toml`:

```toml
[dependencies]
rudi = "0.9"
```

The default features (`rudi-macro` and `auto-register`) are enabled automatically.

## Configuration

No configuration files or environment variables are required. The default feature flags provide a fully functional setup with attribute macros and auto-registration.

To disable auto-registration:

```toml
[dependencies]
rudi = { version = "0.9", default-features = false, features = ["rudi-macro"] }
```

To enable tracing:

```toml
[dependencies]
rudi = { version = "0.9", features = ["tracing"] }
```

## First Run

Create a minimal application with dependency injection:

```rust
use rudi::{Context, Singleton, Transient};

#[derive(Debug, Clone)]
#[Singleton]
struct Config {
    name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { name: "world".into() }
    }
}

#[Transient]
struct Greeter(Config);

fn main() {
    let mut cx = Context::auto_register();
    let greeter = cx.resolve::<Greeter>();
    println!("Hello, {}!", greeter.0.name);
}
```

Expected output:

```
Hello, world!
```

## Next Steps

- [Provider Scopes](../product/features/scopes.md) -- learn about Singleton, Transient, and SingleOwner lifetimes
- [Auto-Registration](../product/features/auto-registration.md) -- understand automatic provider collection
- [Module System](../product/features/modules.md) -- organize providers into reusable modules
- [Async Support](../product/features/async-support.md) -- use async constructors
- [Architecture](../technical/architecture.md) -- understand the internal design
