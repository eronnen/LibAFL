use libafl_structured_mutators::{StructuredMutator, mutators::vec::*};

use crate::MockState;

#[test]
fn test_vec_element_mutator() {
    let mut state = MockState::new(2); // Will mutate third element
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecElementMutator::default();

    // Test mutating third element
    let original = test_vec.clone();
    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_ne!(test_vec[2], original[2]);
    assert_eq!(&test_vec[..2], &original[..2]); // First elements unchanged
    assert_eq!(&test_vec[3..], &original[3..]); // Last elements unchanged

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_clone_insert() {
    // First value (1) selects element to clone, second value (2) selects insert position
    let mut state = MockState::with_rand_values(vec![1, 2]);
    let mut test_vec = vec![1u8, 2, 3];
    let mut mutator = VecCloneInsertMutator::new(0..=1000);

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 4);
    assert_eq!(test_vec, vec![1, 2, 2, 3]); // Second element cloned and inserted at position 2

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_remove() {
    let mut state = MockState::new(1); // Will remove second element
    let mut test_vec = vec![1u8, 2, 3];
    let mut mutator = VecRemoveMutator::new(0..=1000);

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 2);
    assert_eq!(test_vec, vec![1, 3]);

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_swap() {
    let mut state = MockState::with_rand_values(vec![0, 2]); // Will swap indices 0 and 2
    let mut test_vec = vec![1u8, 2, 3];
    let mut mutator = VecSwapMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec, vec![3, 2, 1]);

    // Test with single element
    test_vec = vec![1];
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_reverse_subslice() {
    let mut state = MockState::with_rand_values(vec![1, 3]); // Will reverse elements 1 through 3
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecReverseSubsliceMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec, vec![1, 4, 3, 2, 5]); // Middle section reversed

    // Test with single element
    test_vec = vec![1];
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_rotate_subslice() {
    // First two values (1,3) select subslice, third value (1) selects rotation amount
    let mut state = MockState::with_rand_values(vec![1, 3, 1]);
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecRotateSubsliceMutator::default();

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec, vec![1, 3, 4, 2, 5]); // Middle section rotated by 1

    // Test with single element
    test_vec = vec![1];
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_vec_insert_and_mutate() {
    // First value (1) selects insert position, subsequent values for mutation
    let mut state = MockState::with_rand_values(vec![1, 0, 1]);
    let mut test_vec = vec![1u8, 2, 3];
    let mut mutator = VecInsertAndMutateMutator::new(0..=10);

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 4);
    // A default element (0) was inserted at position 1 and then mutated
    assert_ne!(test_vec[1], 0);
    assert_eq!(test_vec[0], 1);
    assert_eq!(test_vec[2..], vec![2, 3]);

    // Test when vector is at max length
    let mut test_vec = vec![1u8; 11];
    assert!(!mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 11);
}

#[test]
fn test_vec_duplicate_subslice() {
    // Values: start index (1), length (2), insert position (2)
    let mut state = MockState::with_rand_values(vec![1, 1, 2]);
    let mut test_vec = vec![1u8, 2, 3, 4, 5];
    let mut mutator = VecDuplicateSubsliceMutator::new(0..=1000);

    assert!(mutator.mutate(&mut test_vec, &mut state));
    assert_eq!(test_vec.len(), 7);
    assert_eq!(test_vec, vec![1, 2, 2, 3, 3, 4, 5]); // [2,3] duplicated at position 2

    // Test with empty vector
    test_vec.clear();
    assert!(!mutator.mutate(&mut test_vec, &mut state));
}

#[test]
fn test_mutate_empty_vec() {
    let mut state = MockState::new(0);
    libafl_structured_mutators::registry::global_initialize::<u8, MockState>();
    let length_range = 0..=1000;
    let mut mutators: Vec<Box<dyn StructuredMutator<Vec<u8>, MockState>>> = vec![
        Box::new(VecElementMutator::default()),
        Box::new(VecCloneInsertMutator::new(length_range.clone())),
        Box::new(VecRemoveMutator::new(length_range.clone())),
        Box::new(VecSwapMutator::default()),
        Box::new(VecReverseSubsliceMutator::default()),
        Box::new(VecRotateSubsliceMutator::default()),
        Box::new(VecDuplicateSubsliceMutator::new(length_range.clone())),
        Box::new(VecDefaultInsertMutator::new(length_range.clone())),
    ];

    for mutator in &mut mutators {
        let mut test_vec: Vec<u8> = vec![];
        if mutator.weight(&test_vec) == 0 {
            assert!(!mutator.mutate(&mut test_vec, &mut state));
        } else {
            assert!(mutator.mutate(&mut test_vec, &mut state));
            assert!(!test_vec.is_empty());
        }
    }
}
