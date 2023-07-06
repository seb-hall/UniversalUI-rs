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

//  uWindowProvider is implemented by the platform in order to
//  create windows and to enable core functionality such as rendering.
//  It also owns the window objects directly.
pub trait uWindowProvider {

    //  create window and update window handle
    //fn create_window(&self, window: &mut dyn uWindow);
}