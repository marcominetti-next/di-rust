# Auto-Registration

## Overview

Auto-registration allows providers annotated with `#[Singleton]`, `#[Transient]`, or `#[SingleOwner]` to be automatically collected and made available in the context without explicit module declarations. This feature is enabled by the `auto-register` feature flag (on by default) and relies on the `inventory` crate.

## How It Works

```mermaid
flowchart LR
    A["#[Singleton] struct A"] -->|generates| B[DefaultProvider impl]
    B -->|inventory::submit!| C[Global Provider Registry]
    D["#[Transient] fn Foo()"] -->|generates| E[DefaultProvider impl]
    E -->|inventory::submit!| C
    C -->|inventory::iter| F[AutoRegisterModule]
    F -->|providers()| G[Context]
```

## Key Behaviors

- When `auto-register` is enabled, each macro invocation generates an `inventory::submit!` call that registers the provider at link time.
- `Context::auto_register()` creates a context containing all auto-registered providers. It is equivalent to `Context::create(modules![AutoRegisterModule])`.
- Individual providers can opt out with `#[Singleton(auto_register = false)]`.
- Generic types cannot be auto-registered because `inventory` requires a concrete type. Use `auto_register = false` and register them manually in a module.
- For cross-crate auto-registration, each library crate calls `enable! {}` and the consumer calls `lib_crate::enable()` before creating the context.
- The `register_provider!` macro enables auto-registration for providers created with builder functions rather than attribute macros.
- Auto-registration is not available on platforms where the `inventory` crate is unsupported.
