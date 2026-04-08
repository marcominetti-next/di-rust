# Context API Reference

## Overview

The `Context` struct is the central container in Rudi. It stores providers, caches singleton instances, and provides methods for dependency resolution. All public methods are documented here, grouped by category.

## Context Creation

### Context::create

Creates a context from a list of modules.

```rust
let mut cx = Context::create(modules![MyModule]);
```

### Context::auto_register

Creates a context with all auto-registered providers. Requires the `auto-register` feature.

```rust
let mut cx = Context::auto_register();
```

### Context::create_async / Context::auto_register_async

Async variants for contexts containing eager-create async providers.

```rust
let mut cx = Context::create_async(modules![MyModule]).await;
let mut cx = Context::auto_register_async().await;
```

### Context::options

Returns a `ContextOptions` builder for customized context creation.

```rust
let cx = Context::options()
    .eager_create(true)
    .allow_override(false)
    .auto_register();
```

## Resolution Methods (Sync)

| Method | Returns | Panics on missing? |
|--------|---------|-------------------|
| `resolve::<T>()` | `T` | Yes |
| `resolve_with_name::<T>(name)` | `T` | Yes |
| `resolve_option::<T>()` | `Option<T>` | No |
| `resolve_option_with_name::<T>(name)` | `Option<T>` | No |
| `resolve_by_type::<T>()` | `Vec<T>` | No |

All resolve methods work with Singleton and Transient scopes. They panic if the constructor is async.

## Resolution Methods (Async)

| Method | Returns |
|--------|---------|
| `resolve_async::<T>().await` | `T` |
| `resolve_with_name_async::<T>(name).await` | `T` |
| `resolve_option_async::<T>().await` | `Option<T>` |
| `resolve_option_with_name_async::<T>(name).await` | `Option<T>` |
| `resolve_by_type_async::<T>().await` | `Vec<T>` |

## Fallible Resolution (try_ variants)

Each resolve method has a `try_` variant that returns `Result<T, ResolveError>` instead of panicking:

| Method | Returns |
|--------|---------|
| `try_resolve::<T>()` | `Result<T, ResolveError>` |
| `try_resolve_with_name::<T>(name)` | `Result<T, ResolveError>` |
| `try_resolve_option::<T>()` | `Result<Option<T>, ResolveError>` |
| `try_resolve_option_with_name::<T>(name)` | `Result<Option<T>, ResolveError>` |
| `try_resolve_async::<T>().await` | `Result<T, ResolveError>` |
| `try_resolve_with_name_async::<T>(name).await` | `Result<T, ResolveError>` |
| `try_resolve_option_async::<T>().await` | `Result<Option<T>, ResolveError>` |
| `try_resolve_option_with_name_async::<T>(name).await` | `Result<Option<T>, ResolveError>` |

## Singleton/SingleOwner Instance Access

| Method | Returns | Description |
|--------|---------|-------------|
| `get_single::<T>()` | `&T` | Reference to cached instance (panics if missing) |
| `get_single_with_name::<T>(name)` | `&T` | Reference by name (panics if missing) |
| `get_single_option::<T>()` | `Option<&T>` | Optional reference to cached instance |
| `get_single_option_with_name::<T>(name)` | `Option<&T>` | Optional reference by name |
| `get_singles_by_type::<T>()` | `Vec<&T>` | All cached instances of a type |
| `contains_single::<T>()` | `bool` | Whether a cached instance exists |
| `contains_single_with_name::<T>(name)` | `bool` | Whether a named cached instance exists |

## Eager Single Creation

| Method | Returns | Description |
|--------|---------|-------------|
| `just_create_single::<T>()` | `()` | Create and cache without returning (panics if missing) |
| `just_create_single_with_name::<T>(name)` | `()` | Named variant |
| `try_just_create_single::<T>()` | `bool` | Non-panicking variant |
| `try_just_create_single_with_name::<T>(name)` | `bool` | Non-panicking named variant |
| `try_just_create_singles_by_type::<T>()` | `Vec<bool>` | Create all singles of a type |

All have `_async` variants with the same signatures but returning futures.

## Standalone Instance Insertion

| Method | Description |
|--------|-------------|
| `insert_singleton::<T>(instance)` | Insert a pre-built Clone instance as Singleton |
| `insert_singleton_with_name::<T>(instance, name)` | Named variant |
| `insert_single_owner::<T>(instance)` | Insert a pre-built instance as SingleOwner |
| `insert_single_owner_with_name::<T>(instance, name)` | Named variant |

## Provider Inspection

| Method | Returns | Description |
|--------|---------|-------------|
| `contains_provider::<T>()` | `bool` | Whether a provider is registered |
| `contains_provider_with_name::<T>(name)` | `bool` | Named variant |
| `get_provider::<T>()` | `Option<&Provider<T>>` | Reference to a registered provider |
| `get_provider_with_name::<T>(name)` | `Option<&Provider<T>>` | Named variant |
| `get_providers_by_type::<T>()` | `Vec<&Provider<T>>` | All providers of a type |

## Module Management

| Method | Description |
|--------|-------------|
| `load_modules(modules)` | Register providers from additional modules at runtime |
| `unload_modules(modules)` | Remove providers from specified modules |
| `flush()` | Evaluate conditional providers and create eager instances |
| `flush_async().await` | Async variant of flush |

## Context Inspection

| Method | Returns | Description |
|--------|---------|-------------|
| `allow_override()` | `bool` | Whether provider override is enabled |
| `allow_only_single_eager_create()` | `bool` | Whether only singles can be eagerly created |
| `eager_create()` | `bool` | Whether eager creation is enabled |
| `single_registry_len()` | `usize` | Count of cached singleton instances |
| `single_registry_is_empty()` | `bool` | Whether singleton cache is empty |
| `provider_registry_len()` | `usize` | Count of registered providers |
| `provider_registry_is_empty()` | `bool` | Whether provider registry is empty |
| `registry_entries()` | `&HashMap<Key, ProviderEntry>` | Raw registry access |
| `loaded_modules()` | `&HashSet<Type>` | Set of loaded module types |
