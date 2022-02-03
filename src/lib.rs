pub use self::libraw::*;

#[cfg(windows)]
#[path = "windows.rs"]
mod libraw;

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[path = "intel_darwin.rs"]
mod libraw;

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[path = "unix.rs"]
mod libraw;

#[cfg(target_os = "linux")]
#[path = "unix.rs"]
mod libraw;
