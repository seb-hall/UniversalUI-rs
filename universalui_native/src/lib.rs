//  universalui_native crate - src/lib.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  The universalui_native crate communicates directly with
//  native platform APIs such as Win32 and Cocoa to provide
//  abstracted, cross platform such as windowing and file 
//  management, without relying on external dependancies.
//
//  src/lib.rs contains the crate root functions and modules.


//  native module, found in folder structure and structured 
//  identically for common functionality
#[cfg_attr(windows, path = "windows/mod.rs")]
pub mod native;

use crate::native::*;

use universalui_graphics::*;

use universalui_core::window_provider::*;

pub fn native_window_provider() -> uNativeWindowProvider {

    return uNativeWindowProvider::init();
}