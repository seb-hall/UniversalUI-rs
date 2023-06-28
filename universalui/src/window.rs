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

//#[cfg_attr(target_os = "windows", path = "win32/lib.rs")]
//#[cfg_attr(target_os = "linux", path = "linux/lib.rs")]
//mod plm;

use crate::geometry::*;
use crate::string::*;
use crate::windowHandler::*;

use std::rc::Rc;


//  uWindow - a top-level application window
pub struct uWindow {
    pub title: uString,
    pub frame: uRect,
    pub handler: Rc<uWindowHandler>
}

//  uWindow specific functions
impl uWindow {

    //  uWindow init function, takes arguments of title, frame and handler
    pub fn init(title: uString, frame: uRect, handler: uWindowHandler) -> Self {
        let handlerRef = Rc::new(handler);
        //plm::window::NewWindow(frame.width, frame.height, String::from(&title.rust_string[..]), handlerRef.clone());
        use winit::{
            event::{Event, WindowEvent},
            event_loop::EventLoop,
            window::WindowBuilder,
        };
        
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        
        event_loop.run(move |event, _, control_flow| {
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            //control_flow.set_poll();
        
            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            control_flow.set_wait();
        
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    control_flow.set_exit();
                },
                Event::MainEventsCleared => {
                    // Application update code.
        
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    //window.request_redraw();
                },
                Event::RedrawRequested(_) => {
                    // Redraw the application.
                    //
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in MainEventsCleared, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.
                    println!("REDRAW");
                },
                _ => ()
            }
        });


        return uWindow { title: title, frame: frame, handler: handlerRef};
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

