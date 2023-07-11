//  universalui_core crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window.rs defines the window trait

#![allow(non_camel_case_types)]

use crate::string::*;
use crate::geometry::*;
use crate::view::*;
use crate::window_controller::uWindowController;

use std::rc::Rc;
use raw_window_handle::*;

pub struct uWindowHandle {
    pub raw_handle: Option<RawWindowHandle>,
    pub raw_display_handle: Option<RawDisplayHandle>
}

unsafe impl HasRawWindowHandle for uWindowHandle {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        return self.raw_handle.unwrap();
    }
}

unsafe impl HasRawDisplayHandle for uWindowHandle {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        return self.raw_display_handle.unwrap();
    }
}

//  the base 'window' trait
pub struct uWindow {

    //  returns a reference to the title of the window
    pub title: uString,

    //  returns a reference to the size of the window
    pub size: uSize,

    //  window handle, for windowing system
    handle: uWindowHandle,

    //
    controller: Option<Rc<dyn uWindowController>>

    //  returns a reference to the 
    //pub root: dyn uView;

} 

impl uWindow {
    pub fn init(title: &str, size: uSize) -> Self {
        return uWindow { title: uString::init(title), size: size, handle: uWindowHandle { raw_handle: None, raw_display_handle: None }, controller: None};
    }

    pub fn set_handle(&mut self, handle: uWindowHandle) {
        self.handle = handle;
    }

    pub fn get_handle(&self) -> &uWindowHandle {
        return &self.handle;
    }

    pub fn set_window_controller(&mut self, controller: Rc<dyn uWindowController>) {
        //controller = Some(controller);
    }
}