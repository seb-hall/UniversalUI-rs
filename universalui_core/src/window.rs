//  universalui crate - src/window.rs
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

use winit::window::Window;

//  uWindow - a top-level application window
pub struct uWindow {
    pub title: uString,
    pub frame: uRect,
    pub window_handle: Option<Window>
}

//  uWindow specific functions
impl uWindow {

    //  uWindow init function, takes arguments of title, frame and handler
    pub fn init(title: uString, frame: uRect) -> Self {

        return uWindow { title: title, frame: frame, window_handle: None};
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
        return uWindow { title: uString::init("window"), frame: uRect::init(0.0, 0.0, 0.0, 0.0), window_handle: None};
    }
}