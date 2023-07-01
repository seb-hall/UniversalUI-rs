//  universalui crate - src/string.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/string.rs provides core types for strings

#![allow(non_camel_case_types)]

pub struct uString {
    pub raw: String,
}

impl uString {

    //  standard init function from components
    pub fn init(from: &str) -> Self {

        return uString {
            raw: String::from(from),
        };

    }

    pub fn str(&self) -> &str {
        return &self.raw[..];
    }

}