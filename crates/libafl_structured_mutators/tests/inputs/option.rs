use libafl_structured_mutators::StructuredInput;

#[test]
fn test_option_count_and_sample() {
    let some: Option<u8> = Some(42);
    let none: Option<u8> = None;

    // Count data
    assert_eq!(some.count_data::<u8>(), 1);
    assert_eq!(none.count_data::<u8>(), 0);

    // Sample data
    assert_eq!(some.sample_data::<u8>(0), Some(42));
    assert_eq!(some.sample_data::<u8>(1), None);
    assert_eq!(none.sample_data::<u8>(0), None);
}

#[test]
fn test_option_complexity() {
    let some: Option<u8> = Some(7);
    let none: Option<u8> = None;

    // Primitive u8 has complexity 1; Option wraps it so Some -> 1 + inner, None -> 1
    assert_eq!(some.complexity(), 1 + 1);
    assert_eq!(none.complexity(), 1);
}
