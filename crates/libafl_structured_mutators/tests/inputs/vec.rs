use libafl_structured_mutators::StructuredInput;

#[test]
fn test_vec_count_and_sample() {
    let v = vec![1u8, 2u8, 3u8];

    // Vec of primitives should count each element
    assert_eq!(v.count_data::<u8>(), 3);

    // Sample in-bounds
    assert_eq!(v.sample_data::<u8>(0), Some(1));
    assert_eq!(v.sample_data::<u8>(1), Some(2));
    assert_eq!(v.sample_data::<u8>(2), Some(3));

    // Out of bounds
    assert_eq!(v.sample_data::<u8>(3), None);
}

#[test]
fn test_vec_complexity() {
    let v: Vec<u8> = vec![10u8, 20u8];

    // complexity: 1 for vec itself + sum of element complexities (1 each)
    assert_eq!(v.complexity(), 1 + 1 + 1);
}
