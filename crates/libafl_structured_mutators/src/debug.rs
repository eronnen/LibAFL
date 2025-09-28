#[cfg(feature = "tracing-debug")]
macro_rules! debug {
    ($($tt:tt)*) => ($($tt)*)
}

#[cfg(not(feature = "tracing-debug"))]
macro_rules! debug {
    ($($tt:tt)*) => {};
}

pub(crate) use debug;
