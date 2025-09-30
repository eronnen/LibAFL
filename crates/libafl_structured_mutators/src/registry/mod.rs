#![allow(dead_code)]
use core::any::{Any, TypeId};
use std::sync::Once;

use hashbrown::HashMap;

use crate::{HasDefaultStructuredMutator, StructuredInput, StructuredMutator};

#[derive(Debug)]
pub struct TypeMutatorsRegistry<D, S> {
    next_id: u64,
    mutators: HashMap<u64, Box<dyn StructuredMutator<D, S>>>,
}

impl<D, S> Default for TypeMutatorsRegistry<D, S>
where
    D: StructuredInput + HasDefaultStructuredMutator<S>,
{
    fn default() -> Self {
        let mut registry = Self {
            next_id: 0,
            mutators: HashMap::new(),
        };

        registry.insert(D::default_structured_mutator());
        registry
    }
}

impl<D, S> TypeMutatorsRegistry<D, S>
where
    D: StructuredInput + HasDefaultStructuredMutator<S>,
{
    pub fn get<'a>(&'a self, key: u64) -> Option<&'a Box<dyn StructuredMutator<D, S>>> {
        self.mutators.get(&key)
    }

    pub fn insert(&mut self, mutator: Box<dyn StructuredMutator<D, S>>) -> u64 {
        let id = self.next_id;
        self.mutators.insert(id, mutator);
        self.next_id = self.next_id.wrapping_add(1);
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
    pub fn get<D: 'static, S: 'static>(&mut self) -> &TypeMutatorsRegistry<D, S>
    where
        D: StructuredInput + HasDefaultStructuredMutator<S>,
    {
        let key = TypeId::of::<TypeMutatorsRegistry<D, S>>();

        if !self.mutators.contains_key(&key) {
            // Insert a default registry for this type
            let registry = TypeMutatorsRegistry::<D, S>::default();
            self.mutators.insert(key, Box::new(registry));
        }

        self.mutators
            .get(&key)
            .expect("registry entry just inserted")
            .downcast_ref::<TypeMutatorsRegistry<D, S>>()
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
