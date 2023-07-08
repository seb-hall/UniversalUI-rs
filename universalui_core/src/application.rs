#![allow(non_camel_case_types)]

use crate::string::*;
use crate::geometry::*;
use crate::window::*;
use crate::window_provider::*;

use std::rc::Rc;
use std::cell::RefCell;

pub struct uApplicationInformation {
    pub name: uString,
    pub developer: uString,
    pub major_version: i32,
    pub minor_version: i32
}

pub trait uApplication {
    fn set_window_provider(&mut self, provider:  Rc<Box<dyn uWindowProvider>>);
    fn info(&self) -> uApplicationInformation;
    fn finished_launching(&mut self);
    fn window_resized(&self, window: uWindow);
}


pub struct uDesktopApplication {
    name: uString,
    developer: uString,
    major_version: i32,
    minor_version: i32,
    
    window_provider: Option<Rc<Box<dyn uWindowProvider>>>,
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
            window_provider: None,
            windows: Vec::new(),
            finished_launching_callback: Self::default_finished_launching_callback
        };
    }

    

    pub fn add_window(&mut self, mut window: uWindow) {
        let provider = self.window_provider.as_ref().unwrap();
        window.set_handle(provider.create_window(&window));
        self.windows.push(window);
    }

    pub fn set_finished_launching_callback(&mut self, function: fn(app: &mut uDesktopApplication)) {
        self.finished_launching_callback = function;
    }
    
    fn default_finished_launching_callback(_app: &mut uDesktopApplication) { 

    }
}

impl uApplication for uDesktopApplication {

    fn set_window_provider(&mut self, provider: Rc<Box<dyn uWindowProvider>>) {
        self.window_provider = Some(provider);
    }

    fn finished_launching(&mut self) {
        (self.finished_launching_callback)(self);
    }

    fn info(&self) -> uApplicationInformation {
        return uApplicationInformation {
            name: uString::init(self.name.str()),
            developer: uString::init(self.developer.str()),
            major_version: self.major_version.clone(),
            minor_version: self.minor_version.clone()
        }
    }

    fn window_resized(&self, window: uWindow) {
        
    }
}