//! Tests for [`rustix::fs`].

#![cfg(feature = "fs")]
#![cfg(not(windows))]
#![cfg_attr(io_lifetimes_use_std, feature(io_safety))]
#![cfg_attr(core_c_str, feature(core_c_str))]

mod cwd;
mod dir;
mod fcntl;
#[cfg(not(any(
    target_os = "emscripten",
    target_os = "fuchsia",
    target_os = "redox",
    target_os = "wasi"
)))]
mod fcntl_lock;
mod file;
#[cfg(not(target_os = "wasi"))]
mod flock;
mod futimens;
mod invalid_offset;
mod long_paths;
#[cfg(not(any(solarish, target_os = "haiku", target_os = "redox", target_os = "wasi")))]
mod makedev;
mod mkdirat;
mod mknodat;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod openat;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod openat2;
#[cfg(not(target_os = "redox"))]
mod readdir;
mod renameat;
#[cfg(not(any(target_os = "haiku", target_os = "redox", target_os = "wasi")))]
mod statfs;
#[cfg(any(target_os = "android", target_os = "linux"))]
mod statx;
#[cfg(not(any(solarish, target_os = "redox", target_os = "wasi")))]
mod sync;
mod utimensat;
#[cfg(any(apple, target_os = "android", target_os = "linux"))]
mod xattr;
mod y2038;
