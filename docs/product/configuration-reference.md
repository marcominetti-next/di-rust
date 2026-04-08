# Configuration Reference

## Overview

Rudi is configured through Cargo feature flags and the `ContextOptions` builder API. There are no environment variables or configuration files.

## Feature Flags

| Variable | Default | Description |
|----------|---------|-------------|
| `rudi-macro` | Enabled | Enables the `#[Singleton]`, `#[Transient]`, and `#[SingleOwner]` attribute macros |
| `auto-register` | Enabled | Enables automatic provider registration via the `inventory` crate |
| `tracing` | Disabled | Adds structured logging for resolution events via the `tracing` crate |

## Context Options

| Variable | Default | Description |
|----------|---------|-------------|
| `allow_override` | `true` | When true, a provider can replace an existing provider with the same key |
| `allow_only_single_eager_create` | `true` | When true, only Singleton and SingleOwner providers are eligible for eager creation |
| `eager_create` | `false` | When true, all eligible providers are eagerly instantiated during context creation |

## Macro Attribute Arguments

### Provider-Level Arguments

| Variable | Default | Description |
|----------|---------|-------------|
| `name` | `""` | Provider name for disambiguation when multiple providers produce the same type |
| `eager_create` | `false` | Whether to eagerly create the instance during context initialization |
| `condition` | `None` | Optional `fn(&Context) -> bool` predicate controlling whether the provider is registered |
| `binds` | `[]` | Array of transform functions for creating bound providers of derived types |
| `auto_register` | `true` | Whether the provider participates in auto-registration (requires `auto-register` feature) |
| `async` | `false` | Whether the generated constructor is asynchronous (structs and enums only) |

### Field/Argument-Level Arguments

| Variable | Default | Description |
|----------|---------|-------------|
| `name` | `""` | Name of the dependency to resolve from the context |
| `option` | `false` | Resolve as `Option<T>` instead of `T`, returning `None` if not found |
| `default` | `None` | Use a default value if the dependency is not found |
| `vec` | `false` | Resolve all providers of the given type as `Vec<T>` |
| `ref` | `None` | Resolve as a reference `&T` from a Singleton or SingleOwner instance |

### Structural Arguments

| Variable | Default | Description |
|----------|---------|-------------|
| `rudi_path` | `::rudi` | Path to the rudi crate, for use when rudi is re-exported |
