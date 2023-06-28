//  universalui - src/win32/lib.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/win32/lib.rs contains the entry to the win32-specific
//  UniversalUI functionality.

pub mod window;

pub fn init() {
    println!("init win32");
    window::init();
}