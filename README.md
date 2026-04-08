# Rudi

Rudi is an out-of-the-box dependency injection framework for Rust, providing compile-time macro generation and runtime dependency resolution.

## Capabilities

- Three provider scopes: Singleton (cached, cloneable), Transient (fresh per resolve), and SingleOwner (cached, reference-only)
- Attribute macros (`#[Singleton]`, `#[Transient]`, `#[SingleOwner]`) for struct, enum, impl block, and function targets
- Automatic provider registration across compilation units via the `inventory` crate
- Synchronous and asynchronous constructor support with full async resolution API
- Module system for organizing providers with hierarchical submodule support
- Type bindings for registering trait object providers from concrete implementations
- Conditional provider registration based on runtime context state
- Named providers for disambiguating multiple instances of the same type

## Quick Start

1. Add `rudi` to your `Cargo.toml`:

```toml
[dependencies]
rudi = "0.9"
```

2. Annotate your types with scope macros:

```rust
use rudi::{Context, Singleton, Transient};

#[derive(Debug, Clone)]
#[Singleton]
struct Config;

#[Transient]
struct Service(Config);
```

3. Create a context and resolve:

```rust
fn main() {
    let mut cx = Context::auto_register();
    let service = cx.resolve::<Service>();
}
```

## Async Support

Rudi supports async constructors natively. Declare providers with `async fn` or the
`async` attribute and resolve them with `_async` method variants:

```rust
use rudi::{Context, Singleton, Transient};

#[Singleton]
async fn DatabasePool() -> Pool {
    Pool::connect("postgres://localhost/mydb").await
}

#[Transient(async)]
struct Repository(Pool);

#[tokio::main]
async fn main() {
    let mut cx = Context::auto_register_async().await;
    let repo = cx.resolve_async::<Repository>().await;
}
```

## Module System

Organize providers into logical groups using the `Module` trait:

```rust
use rudi::{components, modules, Context, DynProvider, Module, Singleton, Transient};

#[derive(Clone)]
#[Singleton]
struct AppConfig;

#[Transient]
struct Handler(AppConfig);

struct AppModule;

impl Module for AppModule {
    fn providers() -> Vec<DynProvider> {
        components![AppConfig, Handler]
    }
}

fn main() {
    let mut cx = Context::create(modules![AppModule]);
    let handler = cx.resolve::<Handler>();
}
```

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `rudi-macro` feature | Enabled | Enables `#[Singleton]`, `#[Transient]`, `#[SingleOwner]` attribute macros |
| `auto-register` feature | Enabled | Enables automatic provider registration via `inventory` |
| `tracing` feature | Disabled | Adds structured logging for resolution events |
| `allow_override` option | `true` | Whether providers can override existing registrations |
| `allow_only_single_eager_create` option | `true` | Whether only Singleton/SingleOwner providers are eligible for eager creation |
| `eager_create` option | `false` | Whether to eagerly instantiate providers at context creation |

## Documentation

Full documentation is available in the [docs/](docs/index.md) directory:

- [Product Overview](docs/product/overview.md) -- vision, capabilities, and value proposition
- [Getting Started](docs/user/getting-started.md) -- installation, configuration, and first run
- [Architecture](docs/technical/architecture.md) -- crate structure, component design, and key decisions
- [Context API Reference](docs/technical/context-api.md) -- complete Context method reference

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
