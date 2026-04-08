# Architecture

## Overview

Rudi is a Rust dependency injection framework organized as a Cargo workspace with three core crates (`rudi`, `rudi-core`, `rudi-macro`) and three supporting crates (`from-attr`, `from-attr-core`, `from-attr-macro`). The `rudi` crate is the public facade that re-exports types from `rudi-core` and optionally `rudi-macro`. The `Context` struct serves as the central container, managing provider registration, instance caching, and dependency resolution.

## System Context (C4 Level 1)

```mermaid
C4Context
    title Rudi -- System Context

    Person(dev, "Rust Developer", "Annotates types with DI macros, resolves dependencies")

    System(rudi, "Rudi Framework", "Compile-time macro generation + runtime DI container")

    System_Ext(inventory, "inventory", "Link-time provider collection")
    System_Ext(syn, "syn/quote/proc-macro2", "Procedural macro infrastructure")
    System_Ext(tracing, "tracing", "Optional structured logging")

    Rel(dev, rudi, "Uses macros and Context API")
    Rel(rudi, inventory, "Collects auto-registered providers")
    Rel(rudi, syn, "Parses and generates Rust code at compile time")
    Rel(rudi, tracing, "Emits resolution spans")
```

The developer interacts with Rudi through attribute macros at compile time and the `Context` API at runtime. The `inventory` crate enables zero-boilerplate provider collection across compilation units. The `syn` ecosystem powers the procedural macros that generate `DefaultProvider` implementations.

## Container View (C4 Level 2)

```mermaid
flowchart TD
    subgraph "rudi crate (public facade)"
        CTX[Context]
        MOD[Module trait]
        PROV[Provider / DynProvider]
        MACROS[modules! / components! / providers!]
        AR[AutoRegisterModule]
        SINGLE[Single / DynSingle]
    end

    subgraph "rudi-core crate"
        SCOPE[Scope enum]
        COLOR[Color enum]
    end

    subgraph "rudi-macro crate"
        SING_MAC["#[Singleton]"]
        TRANS_MAC["#[Transient]"]
        SO_MAC["#[SingleOwner]"]
    end

    subgraph "from-attr workspace"
        FA[from-attr]
        FAC[from-attr-core]
        FAM[from-attr-macro]
    end

    SING_MAC -->|generates| PROV
    TRANS_MAC -->|generates| PROV
    SO_MAC -->|generates| PROV
    SING_MAC -->|uses| SCOPE
    PROV -->|contains| SCOPE
    PROV -->|contains| COLOR
    MOD -->|returns| PROV
    MACROS -->|creates| MOD
    CTX -->|stores| PROV
    CTX -->|stores| SINGLE
    AR -->|implements| MOD
    SING_MAC -->|parses attrs via| FA
    FA -->|depends on| FAC
    FA -->|depends on| FAM
```

The `rudi` crate contains all runtime types: `Context`, `Provider`, `Module`, `Single`, and the helper macros. The `rudi-macro` crate contains the three attribute proc macros that generate `DefaultProvider` implementations. The `rudi-core` crate holds the `Scope` and `Color` enums shared between the runtime and macro crates. The `from-attr` workspace provides attribute parsing utilities used by `rudi-macro`.

## Component View (C4 Level 3)

```mermaid
flowchart TD
    subgraph Context
        REG[UnifiedRegistry]
        DEP[DependencyChain]
        COND[Conditional Providers]
        EAGER[Eager Create Functions]
        LOADED[Loaded Modules Set]
    end

    subgraph Provider
        DEF[Definition]
        KEY[Key]
        CTOR["Constructor<T>"]
        BIND[Binding Providers]
        ECF[EagerCreateFunction]
    end

    subgraph Resolution
        RESOLVE[resolve / resolve_async]
        TRY_RESOLVE[try_resolve variants]
        GET_SINGLE[get_single / get_single_option]
        BY_TYPE[resolve_by_type]
    end

    REG -->|"HashMap<Key, ProviderEntry>"| KEY
    DEF -->|contains| KEY
    KEY -->|"name + Type"| TYPE[Type]
    TYPE -->|"TypeId + type_name"| TYPEID[std::any::TypeId]

    RESOLVE -->|calls| TRY_RESOLVE
    TRY_RESOLVE -->|looks up| REG
    TRY_RESOLVE -->|checks| DEP
    TRY_RESOLVE -->|invokes| CTOR
    GET_SINGLE -->|reads| REG

    COND -->|evaluated by| FLUSH[flush / flush_async]
    EAGER -->|executed by| FLUSH
```

The `Context` holds a `UnifiedRegistry` that maps `Key` (type + name) to `ProviderEntry` values. Each `ProviderEntry` contains a `DynProvider` and optionally a cached `DynSingle` instance. Resolution traverses the registry, invokes the constructor, detects circular dependencies via `DependencyChain`, and caches singleton results.

## Key Design Decisions

- **Three-crate split**: `rudi-core` holds shared enums so `rudi-macro` can reference `Scope` without depending on the full runtime. This avoids circular dependencies between the proc macro crate and the runtime crate.
- **Type-erased providers**: `DynProvider` erases the generic type parameter of `Provider<T>` using `Box<dyn Any>`, enabling heterogeneous storage in a single `HashMap`. The original `Provider<T>` is recoverable via `as_provider::<T>()`.
- **Unified registry**: Providers and their cached singleton instances are stored together in `ProviderEntry`, eliminating the need for separate provider and instance maps and ensuring consistency.
- **Inventory-based auto-registration**: The `inventory` crate provides a portable, linker-based mechanism for collecting providers across compilation units without requiring a central registration point.
- **Sync/async duality**: Every resolution method has both sync and async variants. The `Color` enum tracks whether a provider's constructor is async, enabling the framework to produce clear errors when async providers are called synchronously.
- **No unsafe code**: The workspace enforces `#![forbid(unsafe_code)]`, relying on `std::any::Any` downcasting instead of unsafe transmutes.
