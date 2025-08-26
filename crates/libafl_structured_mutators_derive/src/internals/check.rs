use crate::internals::{Ctxt, ast::Container};

// Cross-cutting checks that require looking at more than a single attrs object.
// Simpler checks should happen when parsing and building the attrs.
pub fn check(_cx: &Ctxt, _cont: &mut Container) {}
