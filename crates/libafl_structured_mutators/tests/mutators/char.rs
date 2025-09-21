use libafl_structured_mutators::{
    HasDefaultStructuredMutator, StructuredMutator, mutators::char::*,
};

use crate::MockState;

#[test]
fn test_char_inc() {
    let mut state = MockState::new(0);
    let mut test_char = 'A';
    let mut mutator = CharIncMutator::default();

    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, 'B');

    // Test unicode edge case
    test_char = char::from_u32(0x10FFFF).unwrap(); // Max valid unicode
    assert!(!mutator.mutate(&mut test_char, &mut state)); // Should return false
    assert_eq!(test_char, char::from_u32(0x10FFFF).unwrap()); // Should not change
}

#[test]
fn test_char_dec() {
    let mut state = MockState::new(0);
    let mut test_char = 'B';
    let mut mutator = CharDecMutator::default();

    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, 'A');

    // Test unicode edge case
    test_char = '\0';
    assert!(!mutator.mutate(&mut test_char, &mut state)); // Should return false
    assert_eq!(test_char, '\0'); // Should not change
}
}

#[test]
fn test_char_random_ascii() {
    let mut state = MockState::new(65); // Will generate 'A'
    let mut test_char = 'x';
    let mut mutator = CharRandomAsciiMutator::default();

    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, 'A');

    state = MockState::new(97); // Will generate 'a'
    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, 'a');
}

#[test]
fn test_char_random_unicode() {
    let mut state = MockState::new(0x1F600); // Will generate '😀'
    let mut test_char = 'x';
    let mut mutator = CharRandomUnicodeMutator::default();

    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, '😀');

    state = MockState::new(0x3042); // Will generate 'あ'
    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, 'あ');
}

#[test]
fn test_char_printable_ascii() {
    let mut state = MockState::new(0); // Will generate space
    let mut test_char = 'x';
    let mut mutator = CharPrintableAsciiMutator::default();

    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, ' ');

    state = MockState::new(94); // Will generate '~'
    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_eq!(test_char, '~');
}

#[test]
fn test_default_mutator() {
    let mut state = MockState::new(0);
    let mut test_char = 'A';
    let mut mutator = char::default_structured_mutator();

    // Test that we can mutate multiple times
    let original = test_char;
    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_ne!(test_char, original);

    let previous = test_char;
    assert!(mutator.mutate(&mut test_char, &mut state));
    assert_ne!(test_char, previous);
}
