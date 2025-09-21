use libafl::state::HasRand;
use libafl_bolts::rands::Rand;

use crate::{HasDefaultStructuredMutator, StructuredInput, StructuredMutator};

/// Mutator that increments a random character in the string
#[derive(Debug, Default)]
pub struct StringIncCharMutator;

impl<S> StructuredMutator<String, S> for StringIncCharMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut String, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        // Pick a random character index
        let idx = state.rand_mut().below_or_zero(value.len());

        // Get the character at that position
        let mut chars: Vec<char> = value.chars().collect();
        let current = chars[idx];

        // Try to increment the character
        if let Some(new_char) = char::from_u32(current as u32 + 1) {
            chars[idx] = new_char;
            *value = chars.into_iter().collect();
            true
        } else {
            false
        }
    }
}

/// Mutator that decrements a random character in the string
#[derive(Debug, Default)]
pub struct StringDecCharMutator;

impl<S> StructuredMutator<String, S> for StringDecCharMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut String, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        let idx = state.rand_mut().below_or_zero(value.len());
        let mut chars: Vec<char> = value.chars().collect();
        let current = chars[idx];

        if current as u32 > 0 {
            if let Some(new_char) = char::from_u32(current as u32 - 1) {
                chars[idx] = new_char;
                *value = chars.into_iter().collect();
                return true;
            }
        }
        false
    }
}

/// Mutator that changes a random character to a random ASCII character
#[derive(Debug, Default)]
pub struct StringRandomAsciiCharMutator;

impl<S> StructuredMutator<String, S> for StringRandomAsciiCharMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut String, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        let idx = state.rand_mut().below_or_zero(value.len());
        let mut chars: Vec<char> = value.chars().collect();

        // Generate random ASCII value (0-127)
        let random_ascii = state.rand_mut().below_or_zero(128) as u8 as char;
        chars[idx] = random_ascii;

        *value = chars.into_iter().collect();
        true
    }
}

/// Mutator that changes a random character to a random Unicode character
#[derive(Debug, Default)]
pub struct StringRandomUnicodeCharMutator;

impl<S> StructuredMutator<String, S> for StringRandomUnicodeCharMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut String, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        let idx = state.rand_mut().below_or_zero(value.len());
        let mut chars: Vec<char> = value.chars().collect();

        // Try up to 10 times to generate a valid unicode character
        for _ in 0..10 {
            let random_unicode = state.rand_mut().below_or_zero(0x10FFFF + 1) as u32;
            if let Some(new_char) = char::from_u32(random_unicode) {
                chars[idx] = new_char;
                *value = chars.into_iter().collect();
                return true;
            }
        }
        false
    }
}

/// Mutator that inserts a random character at a random position
#[derive(Debug, Default)]
pub struct StringInsertCharMutator;

impl<S> StructuredMutator<String, S> for StringInsertCharMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut String, state: &mut S) -> bool {
        // Can insert at any position, including at the end
        let idx = state.rand_mut().below_or_zero(value.len() + 1);
        let mut chars: Vec<char> = value.chars().collect();

        // Generate a printable ASCII character (32-126)
        let random_char = (state.rand_mut().below_or_zero(95) as u8 + 32) as char;
        chars.insert(idx, random_char);

        *value = chars.into_iter().collect();
        true
    }

    fn weight(&self, data: &String) -> u64 {
        // Higher weight for shorter strings to encourage growth
        if data.len() < 100 {
            2 * data.complexity()
        } else {
            data.complexity()
        }
    }
}

/// Mutator that removes a random character
#[derive(Debug, Default)]
pub struct StringRemoveCharMutator;

impl<S> StructuredMutator<String, S> for StringRemoveCharMutator
where
    S: HasRand,
{
    fn mutate(&mut self, value: &mut String, state: &mut S) -> bool {
        if value.is_empty() {
            return false;
        }

        let idx = state.rand_mut().below_or_zero(value.len());
        let mut chars: Vec<char> = value.chars().collect();
        chars.remove(idx);

        *value = chars.into_iter().collect();
        true
    }

    fn weight(&self, data: &String) -> u64 {
        // Lower weight for shorter strings to prevent excessive shrinking
        if data.len() > 1 {
            data.complexity()
        } else {
            data.complexity() / 2
        }
    }
}

/// Combined string mutator that randomly chooses between different mutation strategies
#[derive(Debug)]
pub struct StringStructuredMutator<S>
where
    S: core::fmt::Debug,
{
    mutators: Vec<Box<dyn StructuredMutator<String, S>>>,
}

impl<S> Default for StringStructuredMutator<S>
where
    S: core::fmt::Debug + HasRand,
{
    fn default() -> Self {
        Self {
            mutators: vec![
                Box::new(StringIncCharMutator::default()),
                Box::new(StringDecCharMutator::default()),
                Box::new(StringRandomAsciiCharMutator::default()),
                Box::new(StringRandomUnicodeCharMutator::default()),
                Box::new(StringInsertCharMutator::default()),
                Box::new(StringRemoveCharMutator::default()),
            ],
        }
    }
}

impl<S> StructuredMutator<String, S> for StringStructuredMutator<S>
where
    S: core::fmt::Debug + HasRand,
{
    fn mutate(&mut self, value: &mut String, state: &mut S) -> bool {
        let mutation = state.rand_mut().choose(&mut self.mutators).unwrap();
        mutation.mutate(value, state)
    }

    fn weight(&self, data: &String) -> u64 {
        // Use maximum weight among all mutators
        self.mutators
            .iter()
            .map(|m| m.weight(data))
            .max()
            .unwrap_or(1)
    }
}

impl<S> HasDefaultStructuredMutator<S> for String
where
    S: core::fmt::Debug + HasRand + 'static,
{
    fn default_structured_mutator() -> Box<dyn StructuredMutator<Self, S>> {
        Box::new(StringStructuredMutator::default())
    }
}
