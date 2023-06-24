//  universalui crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/windowHandler.rs provides access to windowHandler functions

use crate::geometry::*;

//  uWindowHandler - handles window events
#[derive(Clone)]
pub struct uWindowHandler {
    pub window_created: fn(),
    pub window_destroyed: fn(),
    pub window_resized: fn(uSize)
}

//  default window event handlers
fn default_window_created() { println!("default created") }
fn default_window_destroyed() { }
fn default_window_resized(_: uSize) { } 


//  uWindowHandler default implmentation
impl Default for uWindowHandler {
    fn default() -> uWindowHandler {
        uWindowHandler {
            window_created: default_window_created,
            window_destroyed: default_window_destroyed,
            window_resized: default_window_resized
        }   
    }
}