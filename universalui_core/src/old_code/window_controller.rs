//  universalui_core crate - src/window_controller.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window_controller defines the window controller trait.
//  The window controller is resposible for enacting changes to
//  windows and updating the native windowing system.

#![allow(non_camel_case_types)]

use crate::window::*;

pub trait uWindowController {

    //  window resized event
    fn window_resized(&self, window: &mut uWindow);

    //  window created event
    fn window_created(&self, window: &mut uWindow);

    //  create window and update window handle
    //fn create_window(&self, window: &mut uWindow);
    //  set window handle
    //fn set_window_handle(&self, window: &mut uWindow);

    //  show/hide window
    //fn set_window_visibility(&self, window: &mut uWindow, visibility: uWindowVisibility);
    //  set window title
    //fn set_window_title(&self, window: &mut uWindow, title: uString);
    //  set window size
    //fn set_window_size(&self, window: &mut uWindow, size: uSize);

    //fn default_window_handle(&self, ) -> RawWindowHandle;


}
