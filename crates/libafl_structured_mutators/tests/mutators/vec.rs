use libafl_structured_mutators::{
    HasDefaultStructuredMutator, StructuredMutator, mutators::vec::*,
};

use crate::MockState;

#[test]
fn test_vec_element_mutator() {
    let mut state = MockState::new(0);
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecElementMutator::default();

    // Test mutating first element
    let original = test_vec.clone();
    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_ne!(test_vec[0], original[0]);
    assert_eq!(test_vec[1..], original[1..]); // Other elements unchanged

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_clone_insert() {
    let mut state = MockState::new(0); // Will clone first element
    state = MockState::new(0); // Will insert at beginning
    let mut test_vec = vec![1u8, 2, 3];
    let mut mutator = VecCloneInsertMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 4);
    assert_eq!(test_vec[0], 1); // Cloned element inserted at start

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_remove() {
    let mut state = MockState::new(0); // Will remove first element
    let mut test_vec = vec![1u8, 2, 3];
    let mut mutator = VecRemoveMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 2);
    assert_eq!(test_vec, vec![2, 3]);

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_swap() {
    let mut state = MockState::new(0); // Will use indices 0 and 1
    let mut test_vec = vec![1u8, 2, 3];
    let mut mutator = VecSwapMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec, vec![2, 1, 3]);

    // Test with single element
    test_vec = vec![1];
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_reverse_subslice() {
    let mut state = MockState::new(0); // Will use indices 0 and 2
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecReverseSubsliceMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec[0..=2], vec![3, 2, 1]);
    assert_eq!(test_vec[3..], vec![4, 5]);

    // Test with single element
    test_vec = vec![1];
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_rotate_subslice() {
    let mut state = MockState::new(0); // Will use indices 0 and 2
    state = MockState::new(1); // Will rotate by 1
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecRotateSubsliceMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_ne!(test_vec, vec![1, 2, 3, 4, 5]);

    // Test with single element
    test_vec = vec![1];
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_duplicate_subslice() {
    let mut state = MockState::new(0); // Will use start index 0
    state = MockState::new(1); // Will use length 2
    state = MockState::new(0); // Will insert at beginning
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecDuplicateSubsliceMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 7);
    assert_eq!(test_vec[0..2], test_vec[2..4]);

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_default_mutator() {
    let mut state = MockState::new(0);
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = Vec::<u8>::default_structured_mutator();

    // Test multiple mutations
    let original = test_vec.clone();
    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_ne!(test_vec, original);

    let previous = test_vec.clone();
    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_ne!(test_vec, previous);
}
