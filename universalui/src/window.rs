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

#[cfg_attr(target_os = "windows", path = "win32/lib.rs")]
#[cfg_attr(target_os = "linux", path = "linux/lib.rs")]
mod plm;

use crate::geometry::*;
use crate::string::*;
use crate::windowHandler::*;

use std::rc::Rc;


//  uWindow - a top-level application window
pub struct uWindow {
    pub title: uString,
    pub frame: uRect,
    pub handler: Rc<uWindowHandler>
}

//  uWindow specific functions
impl uWindow {

    //  uWindow init function, takes arguments of title, frame and handler
    pub fn init(title: uString, frame: uRect, handler: uWindowHandler) -> Self {
        let handlerRef = Rc::new(handler);
        plm::window::NewWindow(frame.width, frame.height, String::from(&title.rust_string[..]), handlerRef.clone());
        return uWindow { title: title, frame: frame, handler: handlerRef};
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
