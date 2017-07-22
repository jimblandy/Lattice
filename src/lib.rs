#![deny(missing_docs)]
//! A Simple Window Manager for Functional Reactive Programming in Rust

#[macro_use]
extern crate gfx;


/// Visual Components and Modifiers which are to be composed into a single render cycle
pub mod view;

/// Input events and limited object state
pub mod events;

/// Implements the render cycle and exposes a very simple API for creating and manipulating windows
pub mod window;

/// Tools to be used from build.rs scripts
pub mod build;

///Macro to be called from main program to preload all assets into their respective caches
#[macro_export]
macro_rules! with_assets {
   ($w: ident) => (
      $w.load_assets(include!("assets.in").to_vec());
   );
}
