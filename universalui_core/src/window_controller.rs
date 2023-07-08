//  universalui_core crate - src/window_controller.rs
//  created by sebhall on 08/07/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window_controller.rs defines the window controller struct

#![allow(non_camel_case_types)]

use crate::window::*;

//  uWindowProvider is implemented by the platform in order to
//  create windows and to enable core functionality such as rendering.
//  It also owns the window objects directly.
pub trait uWindowController {

    // 
    fn window_resized(&self, window: &mut uWindow);

    //fn window_
    
}