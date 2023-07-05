//  universalui_core crate - src/window_delegate.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window_delegate.rs defines the window delegate struct

#![allow(non_camel_case_types)]

use crate::debug::*;
use crate::geometry::*;
use crate::window::*;
use crate::window_event::*;

use std::vec::Vec;

use raw_window_handle::*;

//  enum definining window configuration
enum uWindowConfiguration {
    single {window: Option<uWindow>}, // used for simple applications
    multiple {windows: Vec<uWindow>} // used for desktop and other applications
}

//  uWindowDelegate directly owns window objects,
//  recieving events for them
pub struct uWindowDelegate {

    //  private enum containing windows
    config: uWindowConfiguration,

}

impl uWindowDelegate {

    //  init function, takes a multiple? parameter. Should be true for desktop
    //  and tool applications, false for single.
    pub fn init(mutiple_windows: bool) -> Self {

        if mutiple_windows {
            return uWindowDelegate {
                config: uWindowConfiguration::multiple{ windows: Vec::new() }
            };
        } else {
            return uWindowDelegate {
                config: uWindowConfiguration::single{ window: None }
            };
        }

    }

    //  Sets the single window for a simple application
    pub fn set_single(&mut self, new_window: uWindow) {
        if let uWindowConfiguration::single { ref mut window } = self.config {
            if window.is_none() {
                self.config = uWindowConfiguration::single { window: Some(new_window) };
            } else {
                debug_warning("simple application tried reassigning single window");
                return;
            }
        } else {
            debug_error("desktop or other application tried to set a simple window!")
        }
    }

    //  Adds a window to the delegate, having been created and a handle generated
    //  by the universalui_native module.
    pub fn add_window(&mut self, window: uWindow) {
        if let uWindowConfiguration::multiple { ref mut windows } = self.config {
            windows.push(window);
        } else {
            debug_error("simple application tried to add a new window to a window delegate!")
        }
    }

    //  Returns mutable vector of windows if application is desktop or other.
    //  Returns none if a simple application. 
    pub fn get_windows(&mut self) -> Option<&mut Vec<uWindow>> {
        if let uWindowConfiguration::multiple { ref mut windows } = self.config {
            return Some(windows);
        } else {
            debug_error("simple application requested desktop windows vector");
            return None;
        }
    }

    //  Returns optional mutable reference to optional main application window. Returns
    //  none if not a simple application.
    pub fn get_window(&mut self) -> Option<&mut Option<uWindow>> {
        if let uWindowConfiguration::single { ref mut window } = self.config {
            return Some(window);
        } else {
            debug_error("desktop or other application requested simple window");
            return None;
        }
    }

    //  TODO - MOVE THIS TO NATIVE AS IT CALLS WIN32 CODE DIRECTLY

    //  Called when a system event occurred for the window
    pub fn event_occurred(&mut self, handle: RawWindowHandle, event: uWindowEvent) {

        if let uWindowConfiguration::single { ref window } = self.config {
            //if window.unwrap().raw_handle.unwrap() == handle {
            //    println!("simple window event occurred!")
            //}
        } else if let uWindowConfiguration::multiple { ref mut windows } = self.config {
            for window in windows {

                if let RawWindowHandle::Win32(existing_handle) = window.raw_handle.unwrap() {
                    if let RawWindowHandle::Win32(sent_handle) = handle {
                        if existing_handle.hwnd == sent_handle.hwnd {
                            println!("event occurred for window {}", window.title.str());
                        }
                    } else {
                        debug_error("failed to get handle from event")
                    }
                } else {
                    debug_error("failed to get handle from existing window")
                }
            }
        } else {
            debug_error("an event occurred for an unknown window!");
        }
    }

}