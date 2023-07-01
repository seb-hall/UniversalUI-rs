//  universalui crate - src/lib.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/app.rs defines the UniversalUI app struct.

#![allow(non_camel_case_types)]

use universalui_core::string::*;
use universalui_core::window::*;
use universalui_core::debug::*;

use std::sync::Arc;

enum uApplicationType {
    simple {window: Option<Arc<uWindow>>},
    desktop {windows: Vec<Arc<uWindow>>},
    other {windows: Vec<Arc<uWindow>>}
}

//  uApplication struct, the entry point of the framework.
//  This should be completed with some basic information 
//  about your app, such as name, version etc.
pub struct uApplication {

    application_type: uApplicationType,

    pub name: uString,
    pub developer: uString,
    pub major_version: i32,
    pub minor_version: i32,
    
    pub finished_launching: fn(),
    pub will_quit: fn()

}

impl uApplication {

    pub fn init_simple(name: &str, developer: &str, major_version: i32, minor_version: i32, finished_launching: fn(), will_quit: fn()) -> Self {
        let app = uApplication {
            application_type: uApplicationType::simple { window: None},
            name: uString::init(name),
            developer: uString::init(developer),
            major_version: major_version,
            minor_version: minor_version,
            finished_launching: finished_launching,
            will_quit: will_quit
        };

        return app;
    }

    pub fn windows(&self) -> Vec<Arc<uWindow>> {
        match &self.application_type {
            uApplicationType::simple { .. } => {
                debug_warning("simple app requested desktop windows, consider changing to a desktop app or using the window() function instead.");
                return Vec::new();
            },
            uApplicationType::desktop { windows } => {
                return windows.clone();
            },
            uApplicationType::other { windows } => {
                return windows.clone();
            }
        }
    }


}