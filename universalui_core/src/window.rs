//  universalui_core crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window.rs provides access to window functions

#![allow(non_camel_case_types)]

use crate::geometry::*;
use crate::string::*;

use raw_window_handle::*;

//  uWindow - a top-level application window
pub struct uWindow {
    pub title: uString,
    pub size: uSize,
    pub raw_handle: Option<RawWindowHandle>
}

//  uWindow specific functions
impl uWindow {

    //  uWindow init function, takes arguments of title, frame and handler
    pub fn init(title: uString, size: uSize) -> Self {

        return uWindow { title: title, size: size, raw_handle: None};
    }

    //  uWindow show function
    pub fn show() {

    }

    //  uWindow hide function
    pub fn hide() {

    }

    //  uWindow maximise function
    pub fn maximise() {

    }

    //  uWindow minimise function
    pub fn minimise() {

    }

}

impl Default for uWindow {
    fn default() -> Self {
        return uWindow { title: uString::init("window"), size: uSize::init(500.0, 300.0), raw_handle: None};
    }
}