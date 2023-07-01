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


pub mod application;
use crate::core::debug::*;

pub fn universalui_init(application: application::uApplication) {
    //#[cfg_attr(target_os = "windows", path = "win32/lib.rs")]
    //#[cfg_attr(target_os = "linux", path = "linux/lib.rs")]
    //mod plm;

    //plm::init();

    debug_info("Welcome to UniversalUI on Rust");
    debug_info(&format!("Initialising '{}' v{}.{}", application.name.str(), application.major_version, application.minor_version)[..]);
    debug_info("this is some info");
    debug_warning("this is a warning");
    debug_error("this is an error");
    debug_critical("this is a critical error");
}