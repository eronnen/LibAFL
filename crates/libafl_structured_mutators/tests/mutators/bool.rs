use libafl_structured_mutators::HasDefaultStructuredMutator;

use crate::MockState;

#[test]
fn test_bool_mutation() {
    let mut state = MockState::new(0);
    let mut test_bool = true;
    let mut mutator = bool::default_structured_mutator();

    // First mutation should flip the boolean
    assert!(mutator.mutate(&mut test_bool, &mut state));
    assert_eq!(test_bool, false);

    // Second mutation should flip it back
    assert!(mutator.mutate(&mut test_bool, &mut state));
    assert_eq!(test_bool, true);
}
