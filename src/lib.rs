#![cfg_attr(feature = "nightly", feature(once_cell))]
#![cfg_attr(feature = "nightly", feature(const_fn_trait_bound))]

#[cfg(all(feature = "stable", feature = "nightly"))]
compile_error!("Both `stable` and `nightly` features are active but they are mutually exclusive");

#[cfg(not(any(feature = "stable", feature = "nightly")))]
compile_error!("Please specify either `stable` or `nightly` features");

#[cfg(feature = "stable")]
mod stable;
#[cfg(feature = "stable")]
pub use stable::*;

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "nightly")]
pub use nightly::*;

#[cfg(test)]
mod tests;