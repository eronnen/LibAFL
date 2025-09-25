/// Expands to its contents only if the `tracing-debug` feature is enabled.
/// Otherwise expands to nothing.
// macro_rules! debug_quote {
//     ($($tt:tt)*) => {
//         {
//             if cfg!(feature = "tracing-debug") {
//                 ::quote::quote! { $($tt)* }
//             } else {
//                 ::proc_macro2::TokenStream::new()
//             }
//         }
//     };
// }

#[cfg(feature = "tracing-debug")]
macro_rules! debug_quote {
    ($($tt:tt)*) => {
        {
            ::quote::quote! { $($tt)* }
        }
    };
}

#[cfg(not(feature = "tracing-debug"))]
macro_rules! debug_quote {
    ($($tt:tt)*) => {
        ::proc_macro2::TokenStream::new()
    };
}

pub(crate) use debug_quote;
