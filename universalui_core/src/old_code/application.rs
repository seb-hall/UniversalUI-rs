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

use universalui_core::geometry::*;
use universalui_core::string::*;
use universalui_core::application_type::*;
use universalui_core::window::*;
use universalui_core::window_delegate;
use universalui_core::window_delegate::*;
use universalui_core::debug::*;

use universalui_native::*;

use core::panic;

#[allow(dead_code)]


/* 

//  uApplication struct, the entry point of the framework.
//  This should be completed with some basic information 
//  about your app, such as name, version etc.
pub struct uApplication {

    //  basic info for the app
    pub name: uString,
    pub developer: uString,
    pub major_version: i32,
    pub minor_version: i32,

    //  preferred window size
    pub preferred_window_size: uSize,

    //  app configuration enum
    app_type: uApplicationType,
    window_delegate: uWindowDelegate,
    
    //  handler functions
    pub finished_launching: fn(sender: &mut uApplication),
    pub will_quit: fn()

}

//  uApplication implementation
impl uApplication {

    //  init_simple function, for initialising simple apps
    pub fn init_simple(name: &str, developer: &str, major_version: i32, minor_version: i32, finished_launching: fn(sender: &mut uApplication), will_quit: fn()) -> Self {
        let app = uApplication {
            preferred_window_size: uSize { width: 100.0, height: 100.0 },
            app_type: uApplicationType::simple,
            window_delegate: uWindowDelegate::init(false),
            name: uString::init(name),
            developer: uString::init(developer),
            major_version: major_version,
            minor_version: minor_version,
            finished_launching: finished_launching,
            will_quit: will_quit
        };

        return app;
    }

    //  init_desktop function, for initialising desktop apps
    pub fn init_desktop(name: &str, developer: &str, major_version: i32, minor_version: i32, finished_launching: fn(sender: &mut uApplication), will_quit: fn()) -> Self {
        let app = uApplication {
            preferred_window_size: uSize { width: 100.0, height: 100.0 },
            app_type: uApplicationType::desktop,
            window_delegate: uWindowDelegate::init(true),
            name: uString::init(name),
            developer: uString::init(developer),
            major_version: major_version,
            minor_version: minor_version,
            finished_launching: finished_launching,
            will_quit: will_quit
        };

        return app;
    }

    //  init_other function, for initialising other apps
    pub fn init_other(name: &str, developer: &str, major_version: i32, minor_version: i32, finished_launching: fn(sender: &mut uApplication), will_quit: fn()) -> Self {
        let app = uApplication {
            preferred_window_size: uSize { width: 100.0, height: 100.0 },
            app_type: uApplicationType::other,
            window_delegate: uWindowDelegate::init(true),
            name: uString::init(name),
            developer: uString::init(developer),
            major_version: major_version,
            minor_version: minor_version,
            finished_launching: finished_launching,
            will_quit: will_quit
        };

        return app;
    }

    



}*/