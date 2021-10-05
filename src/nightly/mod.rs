#[cfg(debug_assertions)]
mod debug;
#[cfg(debug_assertions)]
pub use debug::*;

#[cfg(not(debug_assertions))]
mod release;
#[cfg(not(debug_assertions))]
pub use release::*;