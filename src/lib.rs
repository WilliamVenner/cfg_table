//! [![crates.io](https://img.shields.io/crates/v/cfg_table.svg)](https://crates.io/crates/cfg_table)
//!
//! # ✨ `cfg_table`
//!
//! A simple macro that expands to different values across compilation targets.
//!
//! ## Panics
//!
//! This macro will panic at runtime if no matching value is found.
//!
//! ## Example
//!
//! ```ignore
//! #[macro_use] extern crate cfg_table;
//!
//! let var = cfg_table! {
//!     [all(target_os = "freebsd", target_pointer_width = "64", feature = "my-feature")] => 1337, // custom
//!
//!     // common platforms
//!     win32 => 32,
//!     win64 => 64,
//!     linux32 => 32,
//!     linux64 => 64,
//!     macos32 => 32,
//!     macos64 => 64,
//!
//!     // pointer widths
//!     32 => 1985,
//!     "32" => 1985,
//!     64 => 2003,
//!     "64" => 2003,
//!
//!     _ => 123, // default value if nothing matches, this must be at the bottom
//! };
//!
//! cfg_table! {
//!     win32 => {
//!         println!("You're on Windows 32-bit!");
//!     },
//!
//!     win64 => {
//!         println!("You're on Windows 64-bit!");
//!     },
//!
//!     _ => {
//!         panic!("What the heck is a \"Linux\"?");
//!     },
//! };

#[macro_export]
/// A simple macro that expands to different values across compilation targets.
///
/// **Note, that you must have a leading `,` in the macro invocation.**
///
/// ## Panics
///
/// This macro will panic at runtime if no matching value is found.
///
/// ## Example
///
/// ```rust
/// # use ::cfg_table::cfg_table;
/// let var = cfg_table! {
///     [all(target_os = "freebsd", target_pointer_width = "64", feature = "my-feature")] => 1337, // custom
///
///     // common platforms
///     win32 => 32,
///     win64 => 64,
///     linux32 => 32,
///     linux64 => 64,
///     macos32 => 32,
///     macos64 => 64,
///
///     // pointer widths
///     32 => 1985,
///     "32" => 1985,
///     64 => 2003,
///     "64" => 2003,
///
///     _ => 123, // default value if nothing matches, this must be at the bottom
/// };
///
/// cfg_table! {
///     win32 => {
///         println!("You're on Windows 32-bit!");
///     },
///
///     win64 => {
///         println!("You're on Windows 64-bit!");
///     },
///
///     _ => {
///         panic!("What the heck is a \"Linux\"?");
///     },
/// };
macro_rules! cfg_table {
    () => {
        unreachable!()
    };

    (_ => $expr:expr,) => {{
        $expr
    }};

    ([$cfg:meta] => $expr:expr, $($tail:tt)*) => {{
        #[cfg($cfg)] {
            $expr
        }
        #[cfg(not($cfg))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (32 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(target_pointer_width = "32")] {
            $expr
        }
        #[cfg(not(target_pointer_width = "32"))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (64 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(target_pointer_width = "64")] {
            $expr
        }
        #[cfg(not(target_pointer_width = "64"))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    ("32" => $expr:expr, $($tail:tt)*) => {{
        #[cfg(target_pointer_width = "32")] {
            $expr
        }
        #[cfg(not(target_pointer_width = "32"))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    ("64" => $expr:expr, $($tail:tt)*) => {{
        #[cfg(target_pointer_width = "64")] {
            $expr
        }
        #[cfg(not(target_pointer_width = "64"))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (macos => $expr:expr, $($tail:tt)*) => {{
        #[cfg(target_os = "macos")] {
            $expr
        }
        #[cfg(not(target_os = "macos"))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (linux => $expr:expr, $($tail:tt)*) => {{
        #[cfg(target_os = "linux")] {
            $expr
        }
        #[cfg(not(target_os = "linux"))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (windows => $expr:expr, $($tail:tt)*) => {{
        #[cfg(target_os = "windows")] {
            $expr
        }
        #[cfg(not(target_os = "windows"))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (macos32 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(all(target_os = "macos", target_pointer_width = "32"))] {
            $expr
        }
        #[cfg(not(all(target_os = "macos", target_pointer_width = "32")))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (macos64 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(all(target_os = "macos", target_pointer_width = "64"))] {
            $expr
        }
        #[cfg(not(all(target_os = "macos", target_pointer_width = "64")))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (linux32 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(all(target_os = "linux", target_pointer_width = "32"))] {
            $expr
        }
        #[cfg(not(all(target_os = "linux", target_pointer_width = "32")))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (linux64 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(all(target_os = "linux", target_pointer_width = "64"))] {
            $expr
        }
        #[cfg(not(all(target_os = "linux", target_pointer_width = "64")))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (win32 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(all(target_os = "windows", target_pointer_width = "32"))] {
            $expr
        }
        #[cfg(not(all(target_os = "windows", target_pointer_width = "32")))] {
            $crate::cfg_table!($($tail)*)
        }
    }};

    (win64 => $expr:expr, $($tail:tt)*) => {{
        #[cfg(all(target_os = "windows", target_pointer_width = "64"))] {
            $expr
        }
        #[cfg(not(all(target_os = "windows", target_pointer_width = "64")))] {
            $crate::cfg_table!($($tail)*)
        }
    }};
}