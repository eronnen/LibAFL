use libafl_structured_mutators::{HasDefaultStructuredMutator, StructureMutate};

use crate::MockState;

#[derive(Debug, Default, Clone, PartialEq, StructureMutate)]
struct TestStruct {
    a: i8,
    b: u32,
    c: i64,
}

#[test]
fn test_option_mutator() {
    let mut state = MockState::new(0);
    let mut data: Option<TestStruct> = None;
    let mut mutator = Option::<TestStruct>::default_structured_mutator();

    // Start with None, should become Some with default TestStruct
    assert!(mutator.mutate(&mut data, &mut state));
    assert_eq!(data, Some(TestStruct::default()));

    // Mutate back to None
    assert!(mutator.mutate(&mut data, &mut state));
    assert_eq!(data, None); // Should be None again

    state = MockState::new(1);
    data = Some(TestStruct::default());
    assert!(mutator.mutate(&mut data, &mut state));

    // Should be mutated to a different value
    assert_ne!(data, Some(TestStruct::default()));
    assert!(data.is_some());
}
