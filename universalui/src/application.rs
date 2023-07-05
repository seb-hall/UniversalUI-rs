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

    //  returns mutable vector of windows if application is desktop or other.
    //  returns none if a simple application. 
    pub fn windows(&mut self) -> Option<&mut Vec<uWindow>> {
        return self.window_delegate.get_windows();
    }

    //  window function, returns an optional mutable reference to the simple app window.
    pub fn window(&mut self) -> Option<&mut uWindow> {
        match self.app_type {
            uApplicationType::simple => {
                return self.window_delegate.get_window().unwrap().as_mut();
            },
            uApplicationType::desktop => {
                debug_error("desktop appplication requested simple window, consider changing to a simple appplication or using the windows() function instead.");
                return None;
            },
            uApplicationType::other => {
                debug_error("other appplication requested simple window, consider changing to a simple appplication or using the windows() function instead.");
                return None;
            }
        }
    }

    //  show_window function. This is supported for desktop and other apps.
    //  Calling this function from a simple app will do nothing.
    pub fn show_window(&mut self, mut window: uWindow) {

        match &self.app_type {
            uApplicationType::simple => {
                debug_error("simple appplication tried to add a new window, consider changing to a desktop application.");
                return;
            },
            uApplicationType::desktop => {
                debug_info(&format!("application with name '{}' added a window with name '{}'", self.name.str(), window.title.str())[..]);
                if !native::window::create_window(&mut window, &mut self.window_delegate) {
                    debug_critical("window creation failed! Panicking!");
                    panic!();
                }
                self.window_delegate.add_window(window);
                return;
            },
            uApplicationType::other => {
                debug_info(&format!("application with name '{}' added a window with name '{}'", self.name.str(), window.title.str())[..]);
                if !native::window::create_window(&mut window, &mut self.window_delegate) {
                    debug_critical("window creation failed! Panicking!");
                    panic!();
                }
                self.window_delegate.add_window(window);
                return;
            }
        }
        
    }

    pub fn run(&mut self) {

        if let uApplicationType::simple = self.app_type {

            debug_info("creating simple application window...");

            let mut simple_window = uWindow::init(uString::init(self.name.str()), uSize { width: self.preferred_window_size.width, height: self.preferred_window_size.height });

            if !native::window::create_window(&mut simple_window, &mut self.window_delegate) {
                debug_critical("window creation failed! Panicking!");
                panic!();
            }

            self.window_delegate.set_single(simple_window);
        }

        native::event_loop::run();
    
    }



}