#![allow(dead_code)]
use core::any::{Any, TypeId};
use std::sync::Once;

use hashbrown::HashMap;

use crate::{HasDefaultStructuredMutator, StructuredInput, StructuredMutator};

#[derive(Debug)]
pub struct TypeMutatorsRegistry<D, S> {
    mutators: Vec<Box<dyn StructuredMutator<D, S>>>,
}

impl<D, S> Default for TypeMutatorsRegistry<D, S>
where
    D: StructuredInput + HasDefaultStructuredMutator<S>,
{
    fn default() -> Self {
        let mut registry = Self {
            mutators: Vec::new(),
        };

        // Insert the default mutator as the first entry.
        registry.insert(D::default_structured_mutator());
        registry
    }
}

impl<D, S> TypeMutatorsRegistry<D, S>
where
    D: StructuredInput,
{
    pub fn get<'a>(&'a self, key: u64) -> Option<&'a Box<dyn StructuredMutator<D, S>>> {
        let idx = key as usize;
        self.mutators.get(idx)
    }

    /// Returns a mutable reference to a stored mutator, if present.
    ///
    /// This lets callers borrow a `&mut dyn StructuredMutator<D, S>` for the
    /// duration of the returned borrow. The lifetime is tied to `&mut self`.
    pub fn get_mut(&mut self, key: u64) -> Option<&mut dyn StructuredMutator<D, S>> {
        let idx = key as usize;
        match self.mutators.get_mut(idx) {
            Some(b) => Some(&mut **b),
            None => None,
        }
    }

    pub fn insert(&mut self, mutator: Box<dyn StructuredMutator<D, S>>) -> u64 {
        let id = self.mutators.len() as u64;
        self.mutators.push(mutator);
        id
    }
}

/// A registry of mutators for each structured type.
/// the main usage of this is to re-use mutators between structs and avoid recursion``
#[derive(Debug)]
pub struct MutatorsRegistry {
    mutators: HashMap<TypeId, Box<dyn Any>>,
}

impl MutatorsRegistry {
    /// Return an immutable reference to the per-type registry if it exists.
    /// Does NOT create or insert a default registry.
    pub fn get<D: 'static, S: 'static>(&self) -> Option<&TypeMutatorsRegistry<D, S>>
    where
        D: StructuredInput,
    {
        let key = TypeId::of::<TypeMutatorsRegistry<D, S>>();

        self.mutators.get(&key).and_then(|b| {
            let any_ref: &dyn Any = &**b;
            any_ref.downcast_ref::<TypeMutatorsRegistry<D, S>>()
        })
    }

    /// Mutable variant of `get`. Inserts a default registry if missing and
    /// returns a mutable reference to the per-type registry.
    /// Return a mutable reference to the per-type registry if it exists.
    /// Does NOT create or insert a default registry.
    pub fn get_mut<D: 'static, S: 'static>(&mut self) -> Option<&mut TypeMutatorsRegistry<D, S>>
    where
        D: StructuredInput,
    {
        let key = TypeId::of::<TypeMutatorsRegistry<D, S>>();

        self.mutators.get_mut(&key).and_then(|b| {
            let any_mut: &mut dyn Any = &mut **b;
            any_mut.downcast_mut::<TypeMutatorsRegistry<D, S>>()
        })
    }

    /// Explicitly initialize and insert a default per-type registry for (D, S)
    /// and return a mutable reference to it. This replaces the previous auto-
    /// insert behavior from `get`/`get_mut`.
    pub fn initialize<D: 'static, S: 'static>(&mut self) -> &mut TypeMutatorsRegistry<D, S>
    where
        D: StructuredInput + HasDefaultStructuredMutator<S>,
    {
        let key = TypeId::of::<TypeMutatorsRegistry<D, S>>();

        use hashbrown::hash_map::Entry;

        let boxed_any_mut: &mut Box<dyn Any> = match self.mutators.entry(key) {
            Entry::Vacant(v) => v.insert(Box::new(TypeMutatorsRegistry::<D, S>::default())),
            Entry::Occupied(o) => o.into_mut(),
        };

        boxed_any_mut
            .downcast_mut::<TypeMutatorsRegistry<D, S>>()
            .expect("downcast to concrete registry failed")
    }
}

// ---------------------------------------------------------------------------
// Global registry
// ---------------------------------------------------------------------------

/// Global MutatorsRegistry instance.
///
/// It's lazily initialized on first access and protected by a `Mutex` for safe
/// concurrent access. The value lives for the entire program lifetime and will
/// be dropped at program termination.
// We keep a global, lazily-initialized raw pointer to a `MutatorsRegistry` in
// a `OnceLock`. This avoids `Sync`/`Send` requirements on the stored
// `MutatorsRegistry` (useful for single-threaded usage) while still providing
// a safe initialization protocol.
static mut GLOBAL_MUTATORS_REGISTRY_PTR: *mut MutatorsRegistry = core::ptr::null_mut();
static GLOBAL_MUTATORS_REGISTRY_INIT: Once = Once::new();

/// Returns a mutable reference to the global `MutatorsRegistry`.
///
/// This uses a `static mut` raw pointer and an init `Once` to allocate the
/// registry on first use. This is intentionally single-threaded (no mutex),
/// callers must ensure they don't access it concurrently.
pub fn global_mutators_registry() -> &'static mut MutatorsRegistry {
    unsafe {
        GLOBAL_MUTATORS_REGISTRY_INIT.call_once(|| {
            let boxed = Box::new(MutatorsRegistry {
                mutators: HashMap::new(),
            });
            GLOBAL_MUTATORS_REGISTRY_PTR = Box::into_raw(boxed);
        });

        &mut *GLOBAL_MUTATORS_REGISTRY_PTR
    }
}

/// Convenience immutable accessor. Returns a shared reference to the global
/// registry. This is safe as long as no mutable access happens concurrently.
pub fn global_mutators_registry_ref() -> &'static MutatorsRegistry {
    unsafe {
        GLOBAL_MUTATORS_REGISTRY_INIT.call_once(|| {
            let boxed = Box::new(MutatorsRegistry {
                mutators: HashMap::new(),
            });
            GLOBAL_MUTATORS_REGISTRY_PTR = Box::into_raw(boxed);
        });

        &*GLOBAL_MUTATORS_REGISTRY_PTR
    }
}

/// Global convenience: return an immutable reference to the `StructuredMutator`
/// for the given `key`, if present.
///
/// Safety: this uses the single-threaded global registry. Callers must ensure
/// no concurrent mutable aliasing occurs.
pub fn global_get_mutator<D: 'static, S: 'static>(
    key: u64,
) -> Option<&'static dyn StructuredMutator<D, S>>
where
    D: StructuredInput,
{
    // Obtain the global registry and try to fetch the per-type registry.
    let reg = global_mutators_registry();
    match reg.get::<D, S>() {
        Some(per) => per.get(key).map(|b| &**b),
        None => None,
    }
}

/// Global convenience: return a mutable reference to the `StructuredMutator`
/// for the given `key`, if present.
///
/// Safety: this hands out a `&'static mut` reference to the stored mutator.
/// Use only when you are certain there are no concurrent borrows.
pub fn global_get_mutator_mut<D: 'static, S: 'static>(
    key: u64,
) -> Option<&'static mut dyn StructuredMutator<D, S>>
where
    D: StructuredInput,
{
    let reg = global_mutators_registry();
    match reg.get_mut::<D, S>() {
        Some(per) => per.get_mut(key),
        None => None,
    }
}
