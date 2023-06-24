//  universalui_core - src/string.rs
//  created by sebhall on 24/06/2023
//
//  universalui_core provides the base types common to all
//  parts of the framework, and is a dependency of all other
//  UniversalUI crates. Developers should use the universalui
//  crate rather than this crate directly.
//
//  src/string.rs provides core types for strings

#![allow(non_camel_case_types)]

pub struct uString {
    pub rust_string: String
}

impl uString {

    //  standard init function from components
    pub fn init(from: &str) -> Self {
        return uString {
            rust_string: String::from(from)
        }
    }

}