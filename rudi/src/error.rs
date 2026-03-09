use crate::definition::{Definition, Key};
use std::fmt;

/// Errors that can occur during dependency resolution.
#[derive(Debug)]
pub enum ResolveError {
    /// No provider registered for the requested type and name.
    NoProvider(Key),
    /// Provider exists but is not Singleton or Transient scope (wrong method called).
    NotSingletonOrTransient(Definition),
    /// Provider exists but is not Singleton or SingleOwner scope (wrong method called).
    NotSingletonOrSingleOwner(Definition),
    /// Async provider constructor called in a synchronous resolve context.
    AsyncInSyncContext(Definition),
    /// Circular dependency detected during resolution.
    CircularDependency(String),
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoProvider(key) => write!(f, "no provider registered for: {:?}", key),
            Self::NotSingletonOrTransient(def) => write!(
                f,
                "registered provider is not `Singleton` or `Transient` for: {:?}",
                def
            ),
            Self::NotSingletonOrSingleOwner(def) => write!(
                f,
                "registered provider is not `Singleton` or `SingleOwner` for: {:?}",
                def
            ),
            Self::AsyncInSyncContext(def) => write!(
                f,
                "unable to call an async constructor in a sync context for: {:?}\n\n\
                 please check all the references to the above type, there are 3 scenarios that will be referenced:\n\
                 1. use `Context::resolve_xxx::<Type>(cx)` to get instances of the type, change to `Context::resolve_xxx_async::<Type>(cx).await`.\n\
                 2. use `yyy: Type` as a field of a struct, or a field of a variant of a enum, use `#[Singleton(async)]`, `#[Transient(async)]` or `#[SingleOwner(async)]` on the struct or enum.\n\
                 3. use `zzz: Type` as a argument of a function, add the `async` keyword to the function.",
                def
            ),
            Self::CircularDependency(chain) => {
                write!(f, "circular dependency detected: {}", chain)
            }
        }
    }
}

impl std::error::Error for ResolveError {}
