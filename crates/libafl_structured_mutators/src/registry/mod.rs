#![allow(dead_code)]
use core::any::{Any, TypeId};

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
