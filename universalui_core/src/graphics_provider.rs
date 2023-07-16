//  universalui_core crate - src/graphics_provider.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/graphics_provider.rs defines the uGraphicsProvider trait. 
//  This defines functions that provides references to most of
//  the core components of the UniversalUI framework.

#![allow(non_camel_case_types)]

use crate::window::*;

//  a provider struct - holds a set of functions that are defined
//  by the native
pub struct uGraphicsProvider {
    //  setup graphics provider, return true if successful
    setup: fn(&mut uGraphicsProvider) -> bool,

    //  setup rendering for window
    setup_for_window: fn(&mut uGraphicsProvider, window: &mut uWindow),
    
    //  render window
    render_window: fn(&mut uGraphicsProvider, window: &mut uWindow),
}

//  default setup to avoid optional variables
impl Default for uGraphicsProvider {
    fn default() -> Self {
        return uGraphicsProvider { 
            setup: default_setup, 
            setup_for_window: default_setup_for_window, 
            render_window: default_render_window,
        };
    }
}

//  default functions, do absolutely nothing besides return true for test functions
fn default_setup(_: &mut uGraphicsProvider) -> bool { true }
fn default_setup_for_window(_: &mut uGraphicsProvider, window: &mut uWindow) { }
fn default_render_window(_: &mut uGraphicsProvider, window: &mut uWindow) { }