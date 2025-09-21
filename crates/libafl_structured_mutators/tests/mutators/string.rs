use libafl_structured_mutators::{
    HasDefaultStructuredMutator, StructuredMutator, mutators::string::*,
};

use crate::MockState;

#[test]
fn test_string_inc_char() {
    let mut state = MockState::new(0); // Will select first character
    let mut test_str = String::from("abc");
    let mut mutator = StringIncCharMutator::default();

    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "bbc"); // First 'a' becomes 'b'

    // Test with empty string
    test_str = String::new();
    assert!(!mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "");
}

#[test]
fn test_string_dec_char() {
    let mut state = MockState::new(0); // Will select first character
    let mut test_str = String::from("bcd");
    let mut mutator = StringDecCharMutator::default();

    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "acd"); // First 'b' becomes 'a'

    // Test with empty string
    test_str = String::new();
    assert!(!mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "");
}

#[test]
fn test_string_random_ascii() {
    let mut state = MockState::new(65); // Will generate 'A'
    let mut test_str = String::from("abc");
    let mut mutator = StringRandomAsciiCharMutator::default();

    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str.len(), 3);
    assert!(test_str.chars().all(|c| c.is_ascii()));

    // Test with empty string
    test_str = String::new();
    assert!(!mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "");
}

#[test]
fn test_string_random_unicode() {
    let mut state = MockState::new(0x1F600); // Will generate '😀'
    let mut test_str = String::from("abc");
    let mut mutator = StringRandomUnicodeCharMutator::default();

    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str.chars().count(), 3); // Still 3 characters
    assert!(test_str.chars().any(|c| !c.is_ascii())); // At least one non-ASCII

    // Test with empty string
    test_str = String::new();
    assert!(!mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "");
}

#[test]
fn test_string_insert_char() {
    let mut state = MockState::new(0); // Will generate space (32)
    let mut test_str = String::from("abc");
    let mut mutator = StringInsertCharMutator::default();

    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str.len(), 4);
    assert_eq!(test_str.chars().nth(0), Some(' '));

    // Test insert in empty string
    test_str = String::new();
    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str.len(), 1);
    assert!(test_str.chars().next().unwrap().is_ascii());
}

#[test]
fn test_string_remove_char() {
    let mut state = MockState::new(1); // Remove second character
    let mut test_str = String::from("abc");
    let mut mutator = StringRemoveCharMutator::default();

    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "ac");

    // Test with empty string
    test_str = String::new();
    assert!(!mutator.mutate(&mut test_str, &mut state));
    assert_eq!(test_str, "");
}

#[test]
fn test_string_default_mutator() {
    let mut state = MockState::new(0);
    let mut test_str = String::from("test string");
    let mut mutator = String::default_structured_mutator();

    // Test multiple mutations
    let original = test_str.clone();
    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_ne!(test_str, original);

    let previous = test_str.clone();
    assert!(mutator.mutate(&mut test_str, &mut state));
    assert_ne!(test_str, previous);
}
