//  universalui crate - src/window.rs
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

use crate::geometry::*;
use crate::string::*;

//use winit::dpi::LogicalSize;
//use winit::event::{Event, WindowEvent};
//use winit::event_loop::{ControlFlow, EventLoop};
//use winit::window::*;

use std::sync::Arc;
use std::rc::Rc;

//  uWindow - a top-level application window
pub struct uWindow {
    pub title: uString,
    pub frame: uRect,
    
    //pub handler: Rc<uWindowHandler>
}

/* 
//  uWindow specific functions
impl uWindow {

    //  uWindow init function, takes arguments of title, frame and handler
    pub fn init(title: uString, frame: uRect, handler: uWindowHandler) -> Self {
        let handler_ref = Rc::new(handler);
        //let mut event_loop = EventLoop::new();
        //let window = WindowBuilder::new().build(&event_loop).unwrap();
        
        //window.set_title(title.str());
        //window.set_inner_size(LogicalSize::new(frame.width, frame.height));
        //window.set_visible(true);

        return uWindow { title: title, frame: frame, handler: handler_ref};
    }

    //  uWindow show function
    pub fn show() {

    }

    //  uWindow hide function
    pub fn hide() {

    }

    //  uWindow maximise function
    pub fn maximise() {

    }

    //  uWindow minimise function
    pub fn minimise() {

    }

}

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

*/