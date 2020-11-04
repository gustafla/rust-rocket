//! This is the rust-rocket crate.
//! It is designed to work as a client library for GNU Rocket.

extern crate byteorder;

pub mod client;
pub mod interpolation;
pub mod track;

pub use client::{Event, Rocket, RocketErr};
