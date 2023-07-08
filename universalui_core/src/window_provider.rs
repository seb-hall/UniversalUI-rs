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
use crate::window_controller::uWindowController;

//  uWindowProvider is implemented by the platform in order to
//  create windows and to enable core functionality such as rendering.
//  It also owns the window objects directly.
pub trait uWindowProvider {

    //  initialise window provider, return true if successful
    fn init(&mut self) -> bool;

    //  create window and update window handle
    fn create_window(&self, window: &uWindow) -> uWindowHandle;

    //  set window title
    fn set_window_title(&self, window: &mut uWindow, title: uString);

    fn set_window_controller(&self, window: &mut uWindow, controller: &dyn uWindowController);

    //  run event loop
    fn run_event_loop(&self);
}