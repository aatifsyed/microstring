//! Small, niche-able, stack allocated strings.
//!
//! ```
//! # use microstring::*;
//! const CURRENCY: NanoString = NanoString::new("GBP").unwrap();
//! ```
//!
//! ```compile_fail
//! # use microstring::*;
//! const TOO_BIG: NanoString = NanoString::new("GEEBEEPEE").unwrap();
//! ```
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

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod generated;
pub use generated::*;
