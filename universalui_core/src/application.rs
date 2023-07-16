//  universalui_core crate - src/application.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/application.rs defines the Application trait. This
//  defines functions that provides references to most of
//  the core components of the UniversalUI framework.

#![allow(non_camel_case_types)]

use crate::string::*;
use crate::geometry::*;
use crate::window::*;
use crate::window_provider::*;
use crate::window_controller::*;
use crate::graphics_provider::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct uApplicationInformation {
    pub name: uString,
    pub developer: uString,
    pub major_version: i32,
    pub minor_version: i32
}

pub trait uApplication {

    //  return application info
    fn info(&self) -> uApplicationInformation;

    //  callbacks
    fn finished_launching(&mut self);

    //  get/set graphics provider
    fn get_graphics_provider(&mut self) -> &mut uGraphicsProvider;
    fn set_graphics_provider(&mut self, provider: uGraphicsProvider);

    //  get/set window provider
    fn get_window_provider(&mut self) -> &mut uWindowProvider;
    fn set_window_provider(&mut self, provider: uWindowProvider);
       
    //  get/set window controller for window   
    //fn get_window_controller(&mut self, window: &uWindow) -> &mut Box<dyn uWindowController>;
    fn set_window_controller(&mut self, window: &uWindow, controller: Box<dyn uWindowController>);

}


pub struct uDesktopApplication {
    name: uString,
    developer: uString,
    major_version: i32,
    minor_version: i32,
    
    graphics_provider: uGraphicsProvider,
    window_provider: uWindowProvider,

    windows: Vec<uWindow>,

    finished_launching_callback: fn(app: &mut uDesktopApplication)
}

impl uDesktopApplication {
    pub fn init(name: &str, developer: &str, major_version: i32, minor_version: i32) -> Self {
        return uDesktopApplication {
            name: uString::init(name),
            developer: uString::init(developer),
            major_version: major_version, 
            minor_version: minor_version,
            graphics_provider: uGraphicsProvider::default(),
            window_provider: uWindowProvider::default(),
            windows: Vec::new(),
            finished_launching_callback: Self::default_finished_launching_callback
        };
    }

    pub fn add_window(&mut self, mut window: uWindow) {
        //self.windows.push(value)
        println!("uDesktopApplication add_window");
        (self.window_provider.create_window)(&mut window);
    }

    pub fn set_finished_launching_callback(&mut self, function: fn(app: &mut uDesktopApplication)) {
        self.finished_launching_callback = function;
    }
    
    fn default_finished_launching_callback(_app: &mut uDesktopApplication) { 

    }
}

impl uApplication for uDesktopApplication {

    //  return application info
    fn info(&self) -> uApplicationInformation {
        return uApplicationInformation {
            name: uString::init(self.name.str()),
            developer: uString::init(self.developer.str()),
            major_version: self.major_version.clone(),
            minor_version: self.minor_version.clone()
        }
    }

    //  callbacks
    fn finished_launching(&mut self) {
        (self.finished_launching_callback)(self);
    }

    //  get/set graphics provider
    fn get_graphics_provider(&mut self) -> &mut uGraphicsProvider {
        return &mut self.graphics_provider;
    }

    fn set_graphics_provider(&mut self, provider: uGraphicsProvider) {
        self.graphics_provider = provider;
    }

    //  get/set window provider
    fn get_window_provider(&mut self) -> &mut uWindowProvider {
        return &mut self.window_provider;
    }

    fn set_window_provider(&mut self, provider: uWindowProvider) {
        self.window_provider = provider;
    }
        
    //  get/set window controller for window   
    //fn get_window_controller(&mut self, window: &uWindow) -> &mut Box<dyn uWindowController>;

    fn set_window_controller(&mut self, window: &uWindow, controller: Box<dyn uWindowController>) {

    }

}