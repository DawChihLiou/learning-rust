//! # Publishing a Crate to Crates.io
//!
//! `publising_a_crate_to_createsio` is an example for managing
//! and publishing a crate.
//!
//! `//!` adds documentation to the crate instead of the items in the crate.
//
//! ## Re-exports Example
//!
//! A hypothetical "art" library
//!
//! To use the modules in other crates, they have to access deep hierarchy
//! to use the items. For example:
//!
//! ```
//! use art::kinds::PrimaryColor;
//! use art::utils::mix;
//!
//! fn main() {
//!    let red = PrimaryColor::Red;
//!    let yellow = PrimaryColor::Yellow;
//!    mix(red, yellow);
//! }
//! ```
//! In other words, the internal module structure is not suitable for the
//! use cases. We can use `pub use` to re-export the items and make them
//! easier to use:
//!
//! ```
//! pub use self::kinds::PrimaryColor;
//! pub use self::kinds::SecondaryColor;
//! pub use self::utils::mix;
//! ```
//!
//! Other crates can use your library like this:
//!
//! ```
//! use art::mix;
//! use art::PrimaryColor;
//!
//! fn main() {
//!    // --snip--
//! }
//! ```
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}
/// Adds one to the numbrer given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publishing_a_crate_to_cratesio::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// `///` adds documentation to the crate items.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Run `cargo doc --open` at the project root to see the documentation.
// Run `cargo test` to run the tests in the documentation (Doct-tests).
