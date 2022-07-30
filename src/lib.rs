#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]
#![allow(clippy::approx_constant)]
pub use self::bindings::*;

#[cfg(all(windows, not(feature = "bindgen")))]
#[path = "windows.rs"]
mod bindings;

#[cfg(all(unix, not(feature = "bindgen")))]
#[path = "unix.rs"]
mod bindings;

#[cfg(feature = "bindgen")]
mod bindings;
