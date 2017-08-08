//! A debug crate for rust inspired by NodeJS [debug](https://github.com/visionmedia/debug) module.
//!
//! ## Usage
//! Here is an simple example in examples folder:
//!
//! ```rust
//! #[macro_use]
//! extern crate debug;
//!
//!
//! fn main() {
//!     debug!(666, 33, "aaa");
//!
//!     debug!(vec![1, 2, 3]);
//! }
//! ```
//!
//! Then run it:
//!
//! ```sh
//! DEBUG=* cargo run
//! ```

#[macro_use]
extern crate lazy_static;
extern crate colored;
extern crate globset;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use globset::{Glob, GlobMatcher};
use colored::*;

lazy_static!{
    static ref DEBUG_MATHERS: (Vec<GlobMatcher>, Vec<GlobMatcher>) = ::std::env::var("DEBUG")
        .unwrap_or(String::new())
        .as_str()
        .split(',')
        .fold((vec![], vec![]), |mut acc, s| {
            if &s[0..0] == "-" {
                acc.1.push(Glob::new(&s[1..])
                            .unwrap()
                            .compile_matcher());
            } else  {
                acc.1.push(Glob::new(s)
                            .unwrap()
                            .compile_matcher());
            }
            return acc;
        });
}

fn get_color(s: &str) -> &str {
    let colors = vec![
        "black",
        "red",
        "green",
        "yellow",
        "blue",
        "magenta",
        "cyan",
        "white",
    ];
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish() as usize;
    return colors[hash % colors.len()];
}

pub fn debug_meta(pkg: &str, file: &str, line: u32) {
    print!(
        "{}:{}:L{} ",
        String::from(pkg).color(get_color(pkg)).bold(),
        String::from(file).color(get_color(file)),
        line
    );
}

pub fn is_debug(pkg_name: &str, file: &str) -> bool {
    let meta = format!("{}:{}", pkg_name, file);
    return !DEBUG_MATHERS.1.iter().any(|g| g.is_match(&meta)) &&
        DEBUG_MATHERS.0.iter().any(|g| g.is_match(&meta));
}

/// Debug variables depends on environment variable `DEBUG`, using glob pattern to filter output.
///
/// e.g.
///
/// ```
///   #[macro_use]
/// extern crate debug;
///
/// fn main() {
///     debug!(666, 33, "aaa");
///
///     debug!(vec![1, 2, 3]);
/// }
/// ```
///
/// Then running:
///
/// ```sh
/// DEBUG=*,-not_this cargo run // for *unix
/// // or
/// set DEBUG=*,-not_this; cargo run // for windows
/// // or
/// $env:DEBUG = "*,-not_this"; cargo run // for PowerShell
/// ```
///
#[macro_export]
macro_rules! debug {
    ( $( $x:expr ),* ) => {
        #[cfg(not(feature = "disable"))]
        {
            let pkg_name = env!("CARGO_PKG_NAME");
            let file = file!();
            if $crate::is_debug(pkg_name, file) {
                $crate::debug_meta(pkg_name, file, line!());
                $(
                    print!(" {:?}", $x);
                )*
                print!("\n");
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        debug!("aaa", 123, "bbb");

        debug!(vec![1, 2, 3]);
    }
}
