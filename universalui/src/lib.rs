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

//pub mod native {
//    pub use universalui_native::*;
//}

pub mod graphics {
    pub use universalui_graphics::*;
}

//pub mod application;
pub mod application_window;

use crate::core::debug::*;
use universalui_core::application::*;

//  universalui_init function, this takes a mutable reference
//  to an instance of uApplication and runs the main application
//  logic. Running any functions before this one is called results
//  in undefined behavior. This function will only return when the 
//  application has been quit, and so no code should be run after
//  this in the main function.
pub fn universalui_init(application: uApplication) {

    debug_info("Welcome to UniversalUI on Rust!");

    //  init graphics etc

    //if !native::native_init() {
    //    return;
    //}

    debug_info(&format!("Initialising '{}' v{}.{}...", application.name.str(), application.major_version, application.minor_version)[..]);


    //(application.finished_launching)(&mut application);

    //application.run();

    debug_info("application completed with no issues");

}