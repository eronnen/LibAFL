use core::fmt::Debug;

use libafl::state::HasRand;
use libafl_bolts::rands::Rand;
use libafl_structured_mutators::{HasDefaultStructuredMutator, StructureMutate};

// Fixed value that will be returned by our FixedRand
const RAND_NUM: u64 = 1;

// A simple fixed random number generator for deterministic testing
#[derive(Debug)]
struct FixedRand;

impl Rand for FixedRand {
    fn set_seed(&mut self, _seed: u64) {}
    fn next(&mut self) -> u64 {
        RAND_NUM
    }
}

// Mock state object that implements HasRand with our FixedRand
#[derive(Debug)]
struct MockState {
    rand: FixedRand,
}

impl MockState {
    fn new() -> Self {
        Self { rand: FixedRand }
    }
}

impl HasRand for MockState {
    type Rand = FixedRand;

    fn rand(&self) -> &Self::Rand {
        &self.rand
    }

    fn rand_mut(&mut self) -> &mut Self::Rand {
        &mut self.rand
    }
}

// Test struct with various types to verify mutation behavior
#[derive(Debug, Default, Clone, StructureMutate)]
struct TestStruct {
    a: u8,
    b: i32,
    c: u64,
}

#[test]
fn test_mutate_primitive() {
    let mut state = MockState::new();
    let mut test_u8: u8 = 42;
    let mut mutator = u8::default_structured_mutator();

    // With our FixedRand always returning 1, this should consistently
    // choose the second mutation strategy (increment)
    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 43); // 42 + 1
}

#[test]
fn test_mutate_struct() {
    let mut state = MockState::new();
    let mut test_struct = TestStruct {
        a: 42,
        b: -123,
        c: 9999,
    };
    let mut mutator = TestStruct::default_structured_mutator();

    // First mutation should affect one of the fields based on our FixedRand
    let original = test_struct.clone();
    assert!(mutator.mutate(&mut test_struct, &mut state));

    // At least one field should be different
    assert!(
        test_struct.a != original.a || test_struct.b != original.b || test_struct.c != original.c
    );
}

#[test]
fn test_multiple_mutations() {
    let mut state = MockState::new();
    let mut test_u8: u8 = 42;
    let mut mutator = u8::default_structured_mutator();

    // Apply multiple mutations and verify the value changes each time
    let mut last_value = test_u8;
    for _ in 0..5 {
        println!("Before mutation: {}", test_u8);
        assert!(mutator.mutate(&mut test_u8, &mut state));
        println!("After mutation: {}", test_u8);
        assert_ne!(test_u8, last_value);
        last_value = test_u8;
    }
}

// Test nested struct mutation
#[derive(Debug, Default, Clone, StructureMutate)]
struct NestedTestStruct {
    inner: TestStruct,
    value: i64,
}

#[test]
fn test_nested_struct_mutation() {
    let mut state = MockState::new();
    let mut test_struct = NestedTestStruct {
        inner: TestStruct {
            a: 42,
            b: -123,
            c: 9999,
        },
        value: -9876543210,
    };
    let mut mutator = NestedTestStruct::default_structured_mutator();

    let original = test_struct.clone();
    assert!(mutator.mutate(&mut test_struct, &mut state));

    // Either the inner struct or the value should have changed
    assert!(
        test_struct.inner.a != original.inner.a
            || test_struct.inner.b != original.inner.b
            || test_struct.inner.c != original.inner.c
            || test_struct.value != original.value
    );
}
