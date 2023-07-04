//  universalui crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/debug.rs defines functions for debugging.

use colored::Colorize;

//  information only, no issues
pub fn debug_info(message: &str) {
    println!(
        "{} {}",
        "[UUI-INFO]:".cyan(),
        message
    )
}

//  warning, no significant issues to functionality but
//  non ideal implementation
pub fn debug_warning(message: &str) {
    println!(
        "{} {}",
        "[UUI-WARNING]:".yellow(),
        message
    )
}


//  error, wrong implementation and functionality affected
pub fn debug_error(message: &str) {
    println!(
        "{} {}",
        "[UUI-ERROR]:".bright_yellow(),
        message
    )
}

//  critical error, app about to crash
pub fn debug_critical(message: &str) {
    println!(
        "{} {}",
        "[UUI-CRITICAL]:".bright_red(),
        message
    )
}

