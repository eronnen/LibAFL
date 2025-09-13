// Re-export derive(SerdeAny)
#[cfg(feature = "derive")]
#[expect(unused_imports)]
#[macro_use]
extern crate libafl_structured_mutators_derive;
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use libafl_structured_mutators_derive::*;
