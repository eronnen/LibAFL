use libafl::state::HasRand;
use libafl_bolts::rands::Rand;

use crate::{HasDefaultStructuredMutator, StructuredInput, StructuredMutator};

/// Mutator that mutates a random element in the vector using its own mutator
#[derive(Debug)]
pub struct VecElementMutator<T, S>
where
    T: StructuredInput + HasDefaultStructuredMutator<S>,
    S: core::fmt::Debug,
{
    pub element_mutator: Box<dyn StructuredMutator<T, S>>,
}

impl<T, S> Default for VecElementMutator<T, S>
where
    T: StructuredInput + HasDefaultStructuredMutator<S>,
    S: core::fmt::Debug,
{
    fn default() -> Self {
        Self {
            element_mutator: T::default_structured_mutator(),
        }
    }
}

impl<T, S> StructuredMutator<Vec<T>, S> for VecElementMutator<T, S>
where
    T: StructuredInput + HasDefaultStructuredMutator<S>,
    S: HasRand + core::fmt::Debug,
{
    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        let idx = state.rand_mut().below_or_zero(value.len());
        self.element_mutator.mutate(&mut value[idx], state)
    }

    fn weight(&self, data: &Vec<T>) -> usize {
        // Weight proportional to number of elements
        data.len()
    }
}

/// Mutator that inserts a cloned element at a random position
#[derive(Debug, Default)]
pub struct VecCloneInsertMutator;

impl<T, S> StructuredMutator<Vec<T>, S> for VecCloneInsertMutator
where
    T: StructuredInput + Clone,
    S: HasRand,
{
    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        // Pick a random element to clone and a random position to insert
        let clone_idx = state.rand_mut().below_or_zero(value.len());
        let insert_idx = state.rand_mut().below_or_zero(value.len() + 1);

        let cloned = value[clone_idx].clone();
        value.insert(insert_idx, cloned);
        true
    }

    fn weight(&self, data: &Vec<T>) -> usize {
        if data.len() < 100 {
            2 * data.complexity() // Encourage growth for small vectors
        } else {
            data.complexity()
        }
    }
}

/// Mutator that removes a random element
#[derive(Debug, Default)]
pub struct VecRemoveMutator;

impl<T, S> StructuredMutator<Vec<T>, S> for VecRemoveMutator
where
    T: StructuredInput,
    S: HasRand,
{
    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        let idx = state.rand_mut().below_or_zero(value.len());
        value.remove(idx);
        true
    }

    fn weight(&self, data: &Vec<T>) -> usize {
        if data.len() > 1 {
            data.complexity()
        } else {
            data.complexity() / 2 // Discourage emptying the vector
        }
    }
}

/// Mutator that swaps two random elements
#[derive(Debug, Default)]
pub struct VecSwapMutator;

impl<T, S> StructuredMutator<Vec<T>, S> for VecSwapMutator
where
    T: StructuredInput,
    S: HasRand,
{
    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        if value.len() < 2 {
            return false;
        }

        let idx1 = state.rand_mut().below_or_zero(value.len());
        let mut idx2;
        loop {
            idx2 = state.rand_mut().below_or_zero(value.len());
            if idx1 != idx2 {
                break;
            }
        }

        value.swap(idx1, idx2);
        true
    }
}

/// Mutator that reverses a random subslice of the vector
#[derive(Debug, Default)]
pub struct VecReverseSubsliceMutator;

impl<T, S> StructuredMutator<Vec<T>, S> for VecReverseSubsliceMutator
where
    T: StructuredInput,
    S: HasRand,
{
    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        if value.len() < 2 {
            return false;
        }

        // Pick two random indices for the subslice
        let idx1 = state.rand_mut().below_or_zero(value.len());
        let idx2 = state.rand_mut().below_or_zero(value.len());

        let (start, end) = if idx1 < idx2 {
            (idx1, idx2)
        } else {
            (idx2, idx1)
        };

        value[start..=end].reverse();
        true
    }
}

/// Mutator that rotates a random subslice of the vector
#[derive(Debug, Default)]
pub struct VecRotateSubsliceMutator;

impl<T, S> StructuredMutator<Vec<T>, S> for VecRotateSubsliceMutator
where
    T: StructuredInput,
    S: HasRand,
{
    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        if value.len() < 2 {
            return false;
        }

        // Pick two random indices for the subslice
        let idx1 = state.rand_mut().below_or_zero(value.len());
        let idx2 = state.rand_mut().below_or_zero(value.len());

        let (start, end) = if idx1 < idx2 {
            (idx1, idx2)
        } else {
            (idx2, idx1)
        };

        // Randomly choose rotation amount
        if end > start {
            let rot = state.rand_mut().below_or_zero(end - start + 1);
            value[start..=end].rotate_left(rot);
            true
        } else {
            false
        }
    }
}

/// Mutator that duplicates a random subslice at a random position
#[derive(Debug, Default)]
pub struct VecDuplicateSubsliceMutator;

/// Maximum length of a subslice that can be duplicated
const MAX_DUPLICATE_LENGTH: usize = 10;

impl<T, S> StructuredMutator<Vec<T>, S> for VecDuplicateSubsliceMutator
where
    T: StructuredInput + Clone,
    S: HasRand,
{
    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        // Pick start and length of subslice to duplicate
        let start = state.rand_mut().below_or_zero(value.len());
        let max_len = value.len() - start;
        let len = state
            .rand_mut()
            .below_or_zero(max_len.min(MAX_DUPLICATE_LENGTH))
            + 1; // Limit max duplicate size

        // Pick insertion point
        let insert_at = state.rand_mut().below_or_zero(value.len() + 1);

        // Clone the subslice and insert
        let duplicated: Vec<_> = value[start..start + len].to_vec();
        value.splice(insert_at..insert_at, duplicated);
        true
    }

    fn weight(&self, data: &Vec<T>) -> usize {
        if data.len() < 50 {
            2 * data.complexity() // Encourage duplication for small vectors
        } else {
            data.complexity()
        }
    }
}

/// Combined vector mutator that randomly chooses between different mutation strategies
#[derive(Debug)]
pub struct VecStructuredMutator<T, S>
where
    T: StructuredInput + HasDefaultStructuredMutator<S>,
    S: core::fmt::Debug,
{
    mutators: Vec<Box<dyn StructuredMutator<Vec<T>, S>>>,
}

impl<T, S> Default for VecStructuredMutator<T, S>
where
    T: StructuredInput + HasDefaultStructuredMutator<S> + Clone,
    S: core::fmt::Debug + HasRand + 'static,
{
    fn default() -> Self {
        Self {
            mutators: vec![
                Box::new(VecElementMutator::default()),
                Box::new(VecCloneInsertMutator::default()),
                Box::new(VecRemoveMutator::default()),
                Box::new(VecSwapMutator::default()),
                Box::new(VecReverseSubsliceMutator::default()),
                Box::new(VecRotateSubsliceMutator::default()),
                Box::new(VecDuplicateSubsliceMutator::default()),
            ],
        }
    }
}

impl<T, S> StructuredMutator<Vec<T>, S> for VecStructuredMutator<T, S>
where
    T: StructuredInput + HasDefaultStructuredMutator<S>,
    S: core::fmt::Debug + HasRand,
{
    fn weight(&self, data: &Vec<T>) -> usize {
        // Use maximum weight among all mutators
        self.mutators
            .iter()
            .map(|m| m.weight(data))
            .max()
            .unwrap_or(1)
    }

    fn mutate(&mut self, value: &mut Vec<T>, state: &mut S) -> bool {
        let mutation = state.rand_mut().choose(&mut self.mutators).unwrap();
        mutation.mutate(value, state)
    }
}

impl<T, S> HasDefaultStructuredMutator<S> for Vec<T>
where
    T: StructuredInput + HasDefaultStructuredMutator<S> + Clone,
    S: core::fmt::Debug + HasRand + 'static,
{
    fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>> {
        Box::new(VecStructuredMutator::default())
    }
}
