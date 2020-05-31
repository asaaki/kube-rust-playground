#![forbid(unsafe_code)]
#![warn(rust_2018_idioms)]
#![deny(warnings)]
#![deny(clippy::cargo)]
// workspace might have projects naturally depending on different versions:
#![allow(clippy::multiple_crate_versions)]
// we're not going to release a crate anyway:
#![allow(clippy::cargo_common_metadata)]
#![deny(clippy::pedantic)]
#![deny(clippy::result_unwrap_used)]
#![deny(clippy::panic)]

fn main() {
    println!("Hello, world!");
}
