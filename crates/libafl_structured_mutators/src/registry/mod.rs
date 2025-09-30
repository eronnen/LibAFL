#![allow(dead_code)]
use core::any::{Any, TypeId};

use hashbrown::HashMap;

use crate::{HasDefaultStructuredMutator, StructuredMutator};

#[derive(Debug)]
pub struct TypeMutatorsRegistry<D, S> {
    next_id: u64,
    mutators: HashMap<u64, Box<dyn StructuredMutator<D, S>>>,
}

impl<D, S> Default for TypeMutatorsRegistry<D, S>
where
    D: StructuredMutator + HasDefaultStructuredMutator<S>,
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
    D: StructuredMutator + HasDefaultStructuredMutator<S>,
{
    pub fn get<'a>(&'a self, key: u64) -> &'a Box<dyn StructuredMutator<D, S>> {
        // TODO: complete
    }

    pub fn insert(&mut self, mutator: Box<dyn StructuredMutator<D, S>>) -> u64 {
        // TODO: complete
    }
}

/// A registry of mutators for each structured type.
/// the main usage of this is to re-use mutators between structs and avoid recursion``
#[derive(Debug)]
pub struct MutatorsRegistry {
    mutators: HashMap<TypeId, Box<dyn Any>>,
}

impl MutatorsRegistry {
    pub fn get<D: 'static, S: 'static>(&self) -> &TypeMutatorsRegistry<D, S> {
        // TODO: if the key doesn't exist, insert the default value and return it
        self.mutators
            .get(&TypeId::of::<TypeMutatorsRegistry<D, S>>())?
            .downcast_ref::<TypeMutatorsRegistry<D, S>>()
            .unwrap()
    }
}
