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

use crate::window_controller;
use crate::window_visibility::*;

use crate::window_controller::*;


use crate::geometry::*;
use crate::string::*;

use raw_window_handle::*;

use std::rc::Rc;

//  uWindow - a top-level application window
pub struct uWindow {
    title: uString,
    size: uSize,
    visibility: uWindowVisibility,

    raw_handle: RawWindowHandle,  
    controller: Rc<dyn uWindowController>
}

//  uWindow specific functions
impl uWindow {

    //  uWindow init function, takes arguments of title, frame and handler
    pub fn init(title: uString, size: uSize, controller: Rc<dyn uWindowController>) -> Self {

        return uWindow { title: title, size: size, visibility: uWindowVisibility::visible, controller: controller, raw_handle: controller.default_window_handle()};
    }

    pub fn title(&self) -> &uString {
        return &self.title;
    }

    pub fn size(&self) -> &uSize {
        return &self.size;
    }

    pub fn visibility(&self) -> &uWindowVisibility {
        return &self.visibility;
    }

    pub fn raw_handle(&self) -> &RawWindowHandle {
        return &self.raw_handle;
    }

}