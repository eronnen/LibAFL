use core::fmt::Debug;

use libafl_structured_mutators::{
    HasDefaultStructuredMutator, StructureMutate, StructuredMutator, mutators::integers::*,
};

use crate::MockState;

// Direct mutator tests
#[test]
fn test_bit_flip_mutator() {
    let mut state = MockState::new(3); // Will flip the 4th bit (index 3)
    let mut test_u8: u8 = 0b00000000;
    let mut mutator = U8BitFlipMutator::default();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 0b00001000); // 4th bit should be set

    // Test multi-byte integers
    let mut test_u16: u16 = 0;
    let mut mutator = U16BitFlipMutator::default();
    state = MockState::new(10); // Will flip the 11th bit

    assert!(mutator.mutate(&mut test_u16, &mut state));
    assert_eq!(test_u16, 1 << 10);
}

#[test]
fn test_byte_flip_mutator() {
    // Test single byte value
    let mut state = MockState::new(0); // Only one byte to flip for u8
    let mut test_u8: u8 = 0x00;
    let mut mutator = U8ByteFlipMutator::default();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 0xFF);

    // Test multi-byte integer
    let mut test_u32: u32 = 0x00000000;
    let mut mutator = U32ByteFlipMutator::default();
    state = MockState::new(1); // Flip second byte

    assert!(mutator.mutate(&mut test_u32, &mut state));
    assert_eq!(test_u32, 0x0000FF00);
}

#[test]
fn test_byte_neg_mutator() {
    // Test single byte value
    let mut state = MockState::new(0);
    let mut test_u8: u8 = 0x55; // 0b01010101
    let mut mutator = U8ByteNegMutator::default();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 0xAA); // 0b10101010

    // Test multi-byte integer
    let mut test_u16: u16 = 0x5555;
    let mut mutator = U16ByteNegMutator::default();
    state = MockState::new(1); // Negate second byte

    assert!(mutator.mutate(&mut test_u16, &mut state));
    assert_eq!(test_u16, 0xAA55); // First byte unchanged, second byte negated
}

#[test]
fn test_byte_rand_mutator() {
    // We'll use FixedRand to ensure deterministic random values
    let mut state = MockState::new(0x42); // Random byte will be 0x42
    let mut test_u8: u8 = 0x00;
    let mut mutator = U8ByteRandMutator::default();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 0x42);

    // Test multi-byte integer
    let mut test_u16: u16 = 0x0000;
    let mut mutator = U16ByteRandMutator::default();
    state = MockState::new(0x42); // Position 0, random value 0x42

    assert!(mutator.mutate(&mut test_u16, &mut state));
    assert_eq!(test_u16, 0x0042);
}

#[test]
fn test_byte_add_mutator() {
    // Test single byte addition
    let mut state = MockState::new(1); // Add 1 to the byte
    let mut test_u8: u8 = 0x40;
    let mut mutator = U8ByteAddMutator::default();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 0x41);

    // Test overflow
    let mut test_u8: u8 = 0xFF;
    state = MockState::new(1);

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 0x00); // Should wrap around

    // Test multi-byte integer
    let mut test_u32: u32 = 0x12345678;
    let mut mutator = U32ByteAddMutator::default();
    state = MockState::new((1 << 8) | 1); // Position 1, value 1

    assert!(mutator.mutate(&mut test_u32, &mut state));
    assert_eq!(test_u32, 0x12345778); // Only second byte changed by 1
}

#[test]
fn test_signed_integers() {
    // Test bit flip on signed integer
    let mut state = MockState::new(7); // Flip sign bit
    let mut test_i8: i8 = 42;
    let mut mutator = I8BitFlipMutator::default();

    assert!(mutator.mutate(&mut test_i8, &mut state));
    assert_eq!(test_i8, -86); // 42 with sign bit flipped

    // Test byte negation on signed integer
    let mut test_i16: i16 = 0x1234;
    let mut mutator = I16ByteNegMutator::default();
    state = MockState::new(1); // Negate high byte

    assert!(mutator.mutate(&mut test_i16, &mut state));
    assert_eq!(test_i16, 0xED34u16 as i16); // High byte negated
}

#[test]
fn test_64bit_mutations() {
    // Test bit flip on u64
    let mut state = MockState::new(63); // Flip highest bit
    let mut test_u64: u64 = 0;
    let mut mutator = U64BitFlipMutator::default();

    assert!(mutator.mutate(&mut test_u64, &mut state));
    assert_eq!(test_u64, 1 << 63);

    // Test byte manipulation on u64
    let mut test_u64: u64 = 0x1122334455667788;
    let mut mutator = U64ByteFlipMutator::default();
    state = MockState::new(3); // Flip 4th byte from right

    assert!(mutator.mutate(&mut test_u64, &mut state));
    assert_eq!(test_u64, 0x11223344AA667788);
}

#[test]
fn test_edge_cases() {
    // Test byte add with max value
    let mut state = MockState::new(1); // Position 0, value 1
    let mut test_u8: u8 = 0xFF;
    let mut mutator = U8ByteAddMutator::default();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 0x00); // 0xFF + 1 = 0x100 → 0x00 (with wrapping)    // Test bit flip at edges
    let mut test_i8: i8 = i8::MIN; // 1000_0000 (-128)
    let mut mutator = I8BitFlipMutator::default();
    state = MockState::new(7); // Flip sign bit (bit 7)

    assert!(mutator.mutate(&mut test_i8, &mut state));
    assert_eq!(test_i8, 0); // Flipping sign bit of -128 (1000_0000) gives 0000_0000 (0)
}

// Structured mutation tests

// Test struct with various types to verify mutation behavior
#[derive(Debug, Default, Clone, StructureMutate)]
struct TestStruct {
    a: u8,
    b: i32,
    c: u64,
}

#[test]
fn test_mutate_primitive_inc() {
    // Using rand value 0 should select increment mutation
    let mut state = MockState::new(0);
    let mut test_u8: u8 = 42;
    let mut mutator = u8::default_structured_mutator();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 43); // 42 + 1
}

#[test]
fn test_mutate_primitive_dec() {
    // Using rand value 1 should select decrement mutation
    let mut state = MockState::new(1);
    let mut test_u8: u8 = 42;
    let mut mutator = u8::default_structured_mutator();

    assert!(mutator.mutate(&mut test_u8, &mut state));
    assert_eq!(test_u8, 41); // 42 - 1
}

#[test]
fn test_mutate_struct() {
    let mut state = MockState::new(0);
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
    let mut state = MockState::new(0);
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
    let mut state = MockState::new(0);
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
