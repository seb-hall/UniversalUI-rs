//  universalui_core crate - src/window_provider.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window_provider.rs defines the window provider struct

#![allow(non_camel_case_types)]

use crate::window::*;
use crate::string::*;
use crate::geometry::*;

//  uWindowProvider is implemented by the platform in order to 
//  create and manage native windows.
pub struct uWindowProvider {

    //  setup window provider, return true if successful
    pub setup: fn() -> bool,

    //  create window and update window handle
    pub create_window: fn(window: &mut uWindow), 

    //  destroy window
    pub destroy_window: fn(window: &mut uWindow) -> bool,

    //  set window title
    pub set_window_title: fn(window: &mut uWindow, title: uString),

    //  set window title
    pub set_window_size: fn(window: &mut uWindow, title: uSize),
}

//  default setup to avoid optional variables
impl Default for uWindowProvider {
    fn default() -> Self {
        return uWindowProvider { 
            setup: default_setup, 
            create_window: default_create_window, 
            destroy_window: default_destroy_window, 
            set_window_title: default_set_window_title, 
            set_window_size: default_set_window_size,
        };
    }
}

//  default functions, do absolutely nothing besides return true for test functions
fn default_setup() -> bool { true }
fn default_create_window(window: &mut uWindow) {  }
fn default_destroy_window(window: &mut uWindow) -> bool { true }
fn default_set_window_title(window: &mut uWindow, title: uString) { }
fn default_set_window_size(window: &mut uWindow, size: uSize) { }

