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
use universalui_core::window::*;
use universalui_core::debug::*;

use glfw::*;

use core::panic;

//  uApplicationConfiguration struct, contains window information,
//  dependent on the three different types of uApplication.
//  SIMPLE applications contain a single, automatically created window.
//  The preferred size of this window can be set for some platforms, 
//  notably macOS, Windows and most Linux configurations.
//  DESKTOP applications are recommended for desktop platforms. They can 
//  contain any number of windows, none of which are created automatically. 
//  Instead, an empty vector is initalised.
//  OTHER applications are intended for tools and applets. They lack
//  desktop features such as a menu bar.
enum uApplicationConfiguration {
    simple {window: uWindow, preferred_size: uRect},
    desktop {windows: Vec<uWindow>},
    other {windows: Vec<uWindow>}
}

//  uApplication struct, the entry point of the framework.
//  This should be completed with some basic information 
//  about your app, such as name, version etc.
pub struct uApplication {

    //  basic info for the app
    pub name: uString,
    pub developer: uString,
    pub major_version: i32,
    pub minor_version: i32,

    //  app configuration enum
    app_config: uApplicationConfiguration,
    glfw: Glfw,
    
    //  handler functions
    pub finished_launching: fn(sender: &mut uApplication),
    pub will_quit: fn()

}

//  uApplication implementation
impl uApplication {

    //  init_simple function, for initialising simple apps
    pub fn init_simple(name: &str, developer: &str, major_version: i32, minor_version: i32, preferred_size: uRect, finished_launching: fn(sender: &mut uApplication), will_quit: fn()) -> Self {
        let app = uApplication {
            glfw: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
            app_config: uApplicationConfiguration::simple { window: uWindow::default(), preferred_size: preferred_size},
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
            glfw: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
            app_config: uApplicationConfiguration::desktop { windows: Vec::new() },
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
            glfw: glfw::init(glfw::FAIL_ON_ERRORS).unwrap(),
            app_config: uApplicationConfiguration::other { windows: Vec::new() },
            name: uString::init(name),
            developer: uString::init(developer),
            major_version: major_version,
            minor_version: minor_version,
            finished_launching: finished_launching,
            will_quit: will_quit
        };

        return app;
    }

    //  windows function, returns a reference to the windows vector in
    //  desktop and other apps. Calling this function from a simple app
    //  will cause a panic.
    pub fn windows(&self) -> &Vec<uWindow> {
        match &self.app_config {
            uApplicationConfiguration::simple { .. } => {
                debug_critical("simple appplication requested desktop windows, consider changing to a desktop appplication or using the window() function instead.");
                panic!();
            },
            uApplicationConfiguration::desktop { windows } => {
                return &windows;
            },
            uApplicationConfiguration::other { windows } => {
                return &windows;
            }
        }
    }

    //  window function, returns a reference to the simple app window.
    //  Calling this function from a desktop or other app will cause
    //  a panic.
    pub fn window(&self) -> &uWindow {
        match &self.app_config {
            uApplicationConfiguration::simple { window , ..} => {
                return &window;
            },
            uApplicationConfiguration::desktop { .. } => {
                debug_critical("desktop appplication requested simple window, consider changing to a simple appplication or using the windows() function instead.");
                panic!();
            },
            uApplicationConfiguration::other { .. } => {
                debug_critical("other appplication requested simple window, consider changing to a simple appplication or using the windows() function instead.");
                panic!();
            }
        }
    }

    //  show_window function. This is supported for desktop and other apps.
    //  Calling this function from a simple app will do nothing.
    pub fn show_window(&mut self, mut window: uWindow) {

        match &mut self.app_config {
            uApplicationConfiguration::simple { .. } => {
                debug_error("simple appplication tried to add a new window, consider changing to a desktop application.");
                return;
            },
            uApplicationConfiguration::desktop { windows } => {
                debug_info(&format!("application with name '{}' added a window with name '{}'", self.name.str(), window.title.str())[..]);
                self.glfw.window_hint(glfw::WindowHint::Visible(true));
                let (mut gl_window, events) = self.glfw.create_window(window.frame.width as u32, window.frame.height as u32, window.title.str(), glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");
                gl_window.set_key_polling(true);
                gl_window.make_current();
                window.window_handle = Some((gl_window, events));
                windows.push(window);
                return;
            },
            uApplicationConfiguration::other { windows } => {
                debug_info(&format!("application with name '{}' added a window with name '{}'", self.name.str(), window.title.str())[..]);
                self.glfw.window_hint(glfw::WindowHint::Visible(true));
                let (mut gl_window, events) = self.glfw.create_window(window.frame.width as u32, window.frame.height as u32, window.title.str(), glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");
                gl_window.set_key_polling(true);
                gl_window.make_current();
                window.window_handle = Some((gl_window, events));
                windows.push(window);
                return;
            }
        }
        
    }

    pub fn run(&mut self) {

        match &mut self.app_config {
            uApplicationConfiguration::simple { window, preferred_size } => {
                let mut newWindow = uWindow::default();
                self.glfw.window_hint(glfw::WindowHint::Visible(true));
                let (mut gl_window, events) = self.glfw.create_window(preferred_size.width as u32, preferred_size.height as u32, self.name.str(), glfw::WindowMode::Windowed)
                .expect("Failed to create GLFW window.");
                gl_window.set_key_polling(true);
                gl_window.make_current();
                let (x, y) = gl_window.get_pos();
                let (w, h) = gl_window.get_size();
                newWindow.window_handle = Some((gl_window, events));
                newWindow.title = self.name.clone();
                newWindow.frame = uRect::init(x as f32, y as f32, w as f32, h as f32);
                *window = newWindow;

        
                while !window.window_handle.as_mut().unwrap().0.should_close() {
                    self.glfw.wait_events();
                    println!("event occurred")
                }
            },
            uApplicationConfiguration::desktop { windows } => {
                let mut windows_open: bool = true;

                while windows_open {
                    self.glfw.wait_events();

                    for ref window in &mut *windows {
                        for (_, event) in glfw::flush_messages(&window.window_handle.as_ref().unwrap().1) {
                            handle_event(window, &window.window_handle.as_ref().unwrap().0, event);
                        }
                    }œß
                }

            },
            uApplicationConfiguration::other { windows } => {
                while !windows.is_empty() {
                    self.glfw.wait_events();
                }
            }
        }

        fn handle_event(window: &uWindow, glfw_window: &glfw::Window, event: glfw::WindowEvent) {
            if event == glfw::WindowEvent::Close {
                debug_info("window should close");
            }

            println!("event!")
        }
    
    }



}