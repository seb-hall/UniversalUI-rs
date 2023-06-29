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

pub mod geometry;
pub mod string;
pub mod window;
pub mod view;
pub mod interaction;

pub fn universalui_init() {
    //#[cfg_attr(target_os = "windows", path = "win32/lib.rs")]
    //#[cfg_attr(target_os = "linux", path = "linux/lib.rs")]
    //mod plm;

    //plm::init();

    println!("Welcome to UniversalUI on Rust");
}