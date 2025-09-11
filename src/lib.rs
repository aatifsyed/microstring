//! Small, niche-able, stack allocated strings
//!
//! ```
//! # use microstring::*;
//! # use std::mem::size_of;
//! assert_eq! {
//!     size_of::<NanoString>(),
//!     size_of::<Option<NanoString>>(),
//! }
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod generated;
pub use generated::*;
