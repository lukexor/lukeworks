//! The homepage hero's backdrop, compiled from `src/lukeworks.rs` by `build.rs`.
//!
//! Held as one string of highlighted HTML rather than an image, so it scales at
//! any width and follows the theme through the same `.syntax-highlighting`
//! rules the blog's code blocks use.
//!
//! Unlike [`crate::content`], this is compiled into both targets: the hero is a
//! plain component, and a component body re-runs in the browser during
//! hydration, so a constant missing from the `hydrate` build would leave the
//! backdrop blank there.

include!(concat!(env!("OUT_DIR"), "/backdrop.rs"));
