use libafl_structured_mutators::{StructureMutate, StructuredInput};

// Define test structures
#[derive(Debug, Clone, PartialEq, StructureMutate)]
struct SimpleStruct {
    a: u8,
    b: u16,
    c: u32,
}

#[derive(Debug, Clone, PartialEq, StructureMutate)]
struct NestedStruct {
    x: SimpleStruct,
    y: u8,
    z: SimpleStruct,
}

#[test]
fn test_count_data_simple() {
    let simple = SimpleStruct {
        a: 42,
        b: 1337,
        c: 31337,
    };

    // Count u8 fields - should find 1 (a)
    assert_eq!(simple.count_data::<u8>(), 1);
    // Count u16 fields - should find 1 (b)
    assert_eq!(simple.count_data::<u16>(), 1);
    // Count u32 fields - should find 1 (c)
    assert_eq!(simple.count_data::<u32>(), 1);
    // Count nonexistent type - should find 0
    assert_eq!(simple.count_data::<i64>(), 0);
}

#[test]
fn test_count_data_nested() {
    let nested = NestedStruct {
        x: SimpleStruct { a: 1, b: 2, c: 3 },
        y: 4,
        z: SimpleStruct { a: 5, b: 6, c: 7 },
    };

    // Count u8 fields - should find 2 (x.a and y)
    assert_eq!(nested.count_data::<u8>(), 3);
    // Count u16 fields - should find 1 (x.b)
    assert_eq!(nested.count_data::<u16>(), 2);
    // Count u32 fields - should find 1 (x.c)
    assert_eq!(nested.count_data::<u32>(), 2);
    // Count SimpleStruct - should find 1 (x)
    assert_eq!(nested.count_data::<SimpleStruct>(), 2);
}

#[test]
fn test_sample_data_simple() {
    let simple = SimpleStruct {
        a: 42,
        b: 1337,
        c: 31337,
    };

    // Sample u8 field - should get 42 at index 0
    assert_eq!(simple.sample_data::<u8>(0), Some(42));
    // Out of bounds index should return None
    assert_eq!(simple.sample_data::<u8>(1), None);

    // Sample u16 field - should get 1337 at index 0
    assert_eq!(simple.sample_data::<u16>(0), Some(1337));
    assert_eq!(simple.sample_data::<u16>(1), None);

    // Sample u32 field - should get 31337 at index 0
    assert_eq!(simple.sample_data::<u32>(0), Some(31337));
    assert_eq!(simple.sample_data::<u32>(1), None);

    // Sample nonexistent type - should always get None
    assert_eq!(simple.sample_data::<i64>(0), None);
}

#[test]
fn test_sample_data_nested() {
    let nested = NestedStruct {
        x: SimpleStruct { a: 1, b: 2, c: 3 },
        y: 4,
        z: SimpleStruct { a: 5, b: 6, c: 7 },
    };

    // Sample u8 fields - should get values in order
    assert_eq!(nested.sample_data::<u8>(0), Some(1)); // x.a
    assert_eq!(nested.sample_data::<u8>(1), Some(4)); // y
    assert_eq!(nested.sample_data::<u8>(2), Some(5)); // z.a
    assert_eq!(nested.sample_data::<u8>(3), None); // out of bounds

    // Sample u16 field - should get x.b
    assert_eq!(nested.sample_data::<u16>(0), Some(2));
    assert_eq!(nested.sample_data::<u16>(1), Some(6));
    assert_eq!(nested.sample_data::<u16>(2), None);

    // Sample u32 field - should get x.c
    assert_eq!(nested.sample_data::<u32>(0), Some(3));
    assert_eq!(nested.sample_data::<u32>(1), Some(7));
    assert_eq!(nested.sample_data::<u32>(2), None);

    // Sample SimpleStruct - should get x
    assert_eq!(
        nested.sample_data::<SimpleStruct>(0),
        Some(SimpleStruct { a: 1, b: 2, c: 3 })
    );
    assert_eq!(
        nested.sample_data::<SimpleStruct>(1),
        Some(SimpleStruct { a: 5, b: 6, c: 7 })
    );
    assert_eq!(nested.sample_data::<SimpleStruct>(2), None);
}

#[test]
fn test_complexity() {
    let simple = SimpleStruct {
        a: 42,
        b: 1337,
        c: 31337,
    };
    let simple2 = SimpleStruct {
        a: 12,
        b: 3456,
        c: 7890,
    };
    let nested = NestedStruct {
        x: simple.clone(),
        y: 4,
        z: simple2.clone(),
    };

    // Simple struct should have complexity = 1 + 3 (1 for itself + 1 for each field)
    assert_eq!(simple.complexity(), 4);

    // Nested struct should have complexity = 1 + 2 (1 for itself + 1 for y + simple.complexity())
    assert_eq!(
        nested.complexity(),
        1 + 1 + simple.complexity() + simple2.complexity()
    );
}
