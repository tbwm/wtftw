#![feature(unboxed_closures)]
#![feature(path_ext)]
#[deny(warnings)]
#[macro_use]
#[link]
extern crate log;
#[macro_use]
extern crate bitflags;
extern crate rustc_serialize;

pub mod config;
pub mod core;
pub mod handlers;
pub mod layout;
pub mod util;
pub mod window_manager;
pub mod window_system;
