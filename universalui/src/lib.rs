//  universalui crate - src/lib.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/lib.rs contains the crate root functions and modules.

pub mod core {
    pub use universalui_core::*;
}

pub mod native {   
    pub use universalui_native::*;
}


use std::rc::Rc;

use crate::core::debug::*;
use universalui_core::{application::*, window::*};
use universalui_native::native::uNativeWindowProvider;

//  universalui_init function, this takes a mutable reference
//  to an instance of uApplication and runs the main application
//  logic. Running any functions before this one is called results
//  in undefined behavior. This function will only return when the 
//  application has been quit, and so no code should be run after
//  this in the main function.
pub fn universalui_init(application: &mut dyn uApplication) {

    debug_info("Welcome to UniversalUI on Rust!");

    //  init graphics etc

    let mut window_provider = native::native_window_provider();
    window_provider.create_window = (uNativeWindowProvider::alternative)(&window_provider, window);
    application.set_window_provider(window_provider.make_provider());

    debug_info(&format!("Initialising '{}' v{}.{}...", application.info().name.str(), application.info().major_version, application.info().minor_version)[..]);

    application.finished_launching();

    //window_provider_reference.run_event_loop();

    debug_info("application completed with no issues");

}