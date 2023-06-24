//  universalui_core - src/window.rs
//  created by sebhall on 24/06/2023
//
//  universalui_core provides the base types common to all
//  parts of the framework, and is a dependency of all other
//  UniversalUI crates. Developers should use the universalui
//  crate rather than this crate directly.
//
//  src/window.rs contains the crate root functions and modules.

#![allow(non_camel_case_types)]

use crate::geometry::*;
use crate::string::*;

pub struct uWindow {
    pub title: uString,
    pub frame: uRect,
    pub delegate: uWindowDelegate
}

impl uWindow {

    pub fn init(title: uString, frame: uRect, delegate: uWindowDelegate) -> Self {
        return uWindow { title: title, frame: frame, delegate: delegate };
    }
}

pub struct uWindowDelegate {
    pub window_created: fn(),
    pub window_destroyed: fn(),
    pub window_resized: fn(uSize)
}

fn default_window_created() { }
fn default_window_destroyed() { }
fn default_window_resized(_: uSize) { } 

impl Default for uWindowDelegate {
    fn default() -> uWindowDelegate {
        uWindowDelegate {
            window_created: default_window_created,
            window_destroyed: default_window_destroyed,
            window_resized: default_window_resized
        }   
    }
}