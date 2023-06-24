//  universalui - src/lib.rs
//  created by sebhall on 24/06/2023
//
//  universalui is the entry point of the UniversalUI framework.
//  All functionality is available through this one crate, 
//  regardless of platform.
//
//  src/lib.rs contains the crate root functions and modules.

pub mod core { pub use universalui_core::*; }

pub fn universalui_init() {

    println!("Welcome to UniversalUI on Rust");
}