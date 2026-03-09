use std::{any::TypeId, borrow::Cow, collections::HashMap};

use crate::{DynProvider, DynSingle, Key, Provider};

/// A unified entry holding a provider and optionally its cached singleton instance.
pub enum ProviderEntry {
    /// Provider registered but no singleton instance yet created (or Transient scope).
    Provider(DynProvider),
    /// Singleton/SingleOwner provider with cached instance.
    WithInstance(DynProvider, DynSingle),
}

impl ProviderEntry {
    /// Returns a reference to the provider in this entry.
    fn provider(&self) -> &DynProvider {
        match self {
            ProviderEntry::Provider(p) => p,
            ProviderEntry::WithInstance(p, _) => p,
        }
    }

    /// Returns a reference to the singleton instance, if one has been cached.
    fn instance(&self) -> Option<&DynSingle> {
        match self {
            ProviderEntry::Provider(_) => None,
            ProviderEntry::WithInstance(_, s) => Some(s),
        }
    }

    /// Returns true if a singleton instance has been cached.
    fn has_instance(&self) -> bool {
        matches!(self, ProviderEntry::WithInstance(_, _))
    }
}

/// A unified registry holding both providers and their cached singleton instances in one map.
#[derive(Default)]
pub(crate) struct UnifiedRegistry {
    entries: HashMap<Key, ProviderEntry>,
    type_index: HashMap<TypeId, Vec<Cow<'static, str>>>,
}

impl UnifiedRegistry {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(capacity),
            type_index: HashMap::with_capacity(capacity),
        }
    }

    pub(crate) fn reserve(&mut self, additional: usize) {
        self.entries.reserve(additional);
        self.type_index.reserve(additional);
    }

    /// Returns the entries map (for public API compatibility).
    pub(crate) fn entries(&self) -> &HashMap<Key, ProviderEntry> {
        &self.entries
    }

    /// Returns all provider names registered for a given TypeId. O(1) lookup.
    pub(crate) fn names_by_type(&self, type_id: TypeId) -> &[Cow<'static, str>] {
        self.type_index
            .get(&type_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Insert a provider (without an instance). Used during module loading.
    #[track_caller]
    pub(crate) fn insert_provider(&mut self, provider: DynProvider, allow_override: bool) {
        let definition = provider.definition();
        let key = provider.key().clone();

        let is_new = !self.entries.contains_key(&key);

        if is_new {
            #[cfg(feature = "tracing")]
            tracing::debug!("(+) insert new: {:?}", definition);
        } else if allow_override {
            #[cfg(feature = "tracing")]
            tracing::warn!("(!) override by `key`: {:?}", definition);
        } else {
            panic!(
                "already existing a provider with the same `key`: {:?}",
                definition
            );
        }

        // Only add to type_index if this is a genuinely new key (not an override)
        if is_new {
            self.type_index
                .entry(key.ty.id)
                .or_default()
                .push(key.name.clone());
        }

        self.entries.insert(key, ProviderEntry::Provider(provider));
    }

    /// Insert a provider together with its singleton instance (for pre-created singletons).
    #[track_caller]
    pub(crate) fn insert_with_instance(
        &mut self,
        provider: DynProvider,
        single: DynSingle,
        allow_override: bool,
    ) {
        let definition = provider.definition();
        let key = provider.key().clone();

        let is_new = !self.entries.contains_key(&key);

        if is_new {
            #[cfg(feature = "tracing")]
            tracing::debug!("(+) insert new with instance: {:?}", definition);
        } else if allow_override {
            #[cfg(feature = "tracing")]
            tracing::warn!("(!) override by `key`: {:?}", definition);
        } else {
            panic!(
                "already existing a provider with the same `key`: {:?}",
                definition
            );
        }

        if is_new {
            self.type_index
                .entry(key.ty.id)
                .or_default()
                .push(key.name.clone());
        }

        self.entries
            .insert(key, ProviderEntry::WithInstance(provider, single));
    }

    /// Attach a singleton instance to an existing entry.
    /// The key must already exist. Transitions Provider -> WithInstance,
    /// or replaces the instance in an existing WithInstance.
    pub(crate) fn set_instance(&mut self, key: Key, single: DynSingle) {
        // Remove and re-insert to avoid needing a placeholder value
        if let Some(entry) = self.entries.remove(&key) {
            let provider = match entry {
                ProviderEntry::Provider(p) => p,
                ProviderEntry::WithInstance(p, _) => p,
            };
            self.entries
                .insert(key, ProviderEntry::WithInstance(provider, single));
        }
    }

    /// Get a typed provider reference by key.
    pub(crate) fn get_provider<T: 'static>(&self, key: &Key) -> Option<&Provider<T>> {
        self.entries.get(key)?.provider().as_provider()
    }

    /// Get a typed singleton reference by key.
    pub(crate) fn get_single_ref<T: 'static>(&self, key: &Key) -> Option<&T> {
        Some(
            self.entries
                .get(key)?
                .instance()?
                .as_single::<T>()?
                .get_ref(),
        )
    }

    /// Returns true if a provider with this key exists.
    pub(crate) fn contains_provider(&self, key: &Key) -> bool {
        self.entries.contains_key(key)
    }

    /// Returns true if a singleton instance with this key exists.
    pub(crate) fn contains_instance(&self, key: &Key) -> bool {
        self.entries
            .get(key)
            .map(|e| e.has_instance())
            .unwrap_or(false)
    }

    /// Get the full entry for use in before_resolve (single-lookup path).
    pub(crate) fn get_entry(&self, key: &Key) -> Option<&ProviderEntry> {
        self.entries.get(key)
    }

    /// Remove a key entirely (provider + instance).
    pub(crate) fn remove(&mut self, key: &Key) -> Option<ProviderEntry> {
        let removed = self.entries.remove(key)?;

        // Update type_index: remove the name entry for this key's type
        if let Some(names) = self.type_index.get_mut(&key.ty.id) {
            if let Some(pos) = names.iter().position(|n| *n == key.name) {
                names.swap_remove(pos);
            }
            if names.is_empty() {
                self.type_index.remove(&key.ty.id);
            }
        }

        Some(removed)
    }
}
