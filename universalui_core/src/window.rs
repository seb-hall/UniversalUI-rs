//  universalui_core crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window.rs defines the window trait

#![allow(non_camel_case_types)]

use crate::string::*;
use crate::geometry::*;
use crate::view::*;

//  the base 'window' trait
pub struct uWindow {

    //  returns a reference to the title of the window
    pub title: uString,

    //  returns a reference to the size of the window
    pub size: uSize,

    //  returns a reference to the 
    //pub root: dyn uView;

}