use libafl::state::HasRand;
use libafl_bolts::rands::Rand;

use crate::{HasDefaultStructuredMutator, StructuredMutator};

/// Mutator that increments a character's unicode value
#[derive(Debug, Default)]
pub struct CharIncMutator;

impl<S> StructuredMutator<char, S> for CharIncMutator {
    fn mutate(&mut self, value: &mut char, _state: &mut S) -> bool {
        let mut as_u32 = *value as u32;
        as_u32 = as_u32.wrapping_add(1);
        // Only update if it's a valid char
        if let Some(new_char) = char::from_u32(as_u32) {
            *value = new_char;
            true
        } else {
            false
        }
    }
}

/// Mutator that decrements a character's unicode value
#[derive(Debug, Default)]
pub struct CharDecMutator;

impl<S> StructuredMutator<char, S> for CharDecMutator {
    fn mutate(&mut self, value: &mut char, _state: &mut S) -> bool {
        let mut as_u32 = *value as u32;
        as_u32 = as_u32.wrapping_sub(1);
        // Only update if it's a valid char
        if let Some(new_char) = char::from_u32(as_u32) {
            *value = new_char;
            true
        } else {
            false
        }
    }
}

/// Mutator that changes a character to a random ASCII character
#[derive(Debug, Default)]
pub struct CharRandomAsciiMutator;

impl<S> StructuredMutator<char, S> for CharRandomAsciiMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut char, state: &mut S) -> bool {
        // Generate random ASCII value (0-127)
        let random_ascii = state.rand_mut().below_or_zero(128) as u8;
        *value = random_ascii as char;
        true
    }
}

/// Mutator that changes a character to a random valid Unicode character
#[derive(Debug, Default)]
pub struct CharRandomUnicodeMutator;

impl<S> StructuredMutator<char, S> for CharRandomUnicodeMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut char, state: &mut S) -> bool {
        // Try up to 10 times to generate a valid unicode character
        for _ in 0..10 {
            // Generate random value up to max valid unicode codepoint (0x10FFFF)
            let random_unicode = state.rand_mut().below_or_zero(0x10FFFF + 1) as u32;

            // Check if it's a valid unicode scalar value
            if let Some(new_char) = char::from_u32(random_unicode) {
                *value = new_char;
                return true;
            }
        }
        false
    }
}

/// Mutator that changes a character to a printable ASCII character
#[derive(Debug, Default)]
pub struct CharPrintableAsciiMutator;

impl<S> StructuredMutator<char, S> for CharPrintableAsciiMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut char, state: &mut S) -> bool {
        // Generate random printable ASCII value (32-126)
        let random_ascii = state.rand_mut().below_or_zero(95) as u8 + 32;
        *value = random_ascii as char;
        true
    }
}

#[derive(Debug)]
pub struct CharStructuredMutator<S>
where
    S: core::fmt::Debug,
{
    mutators: Vec<Box<dyn StructuredMutator<char, S>>>,
}

impl<S> Default for CharStructuredMutator<S>
where
    S: core::fmt::Debug + HasRand,
{
    fn default() -> Self {
        Self {
            mutators: vec![
                Box::new(CharIncMutator::default()),
                Box::new(CharDecMutator::default()),
                Box::new(CharRandomAsciiMutator::default()),
                Box::new(CharRandomUnicodeMutator::default()),
                Box::new(CharPrintableAsciiMutator::default()),
            ],
        }
    }
}

impl<S> StructuredMutator<char, S> for CharStructuredMutator<S>
where
    S: core::fmt::Debug + HasRand,
{
    fn mutate(&mut self, value: &mut char, state: &mut S) -> bool {
        let mutation = state.rand_mut().choose(&mut self.mutators).unwrap();
        mutation.mutate(value, state)
    }
}

// Default structured mutator for char type
impl<S> HasDefaultStructuredMutator<S> for char
where
    S: core::fmt::Debug + HasRand + 'static,
{
    fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>> {
        Box::new(CharStructuredMutator::default())
    }
}
