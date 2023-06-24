//  universalui_core - src/geometry.rs
//  created by sebhall on 24/06/2023
//
//  universalui_core provides the base types common to all
//  parts of the framework, and is a dependency of all other
//  UniversalUI crates. Developers should use the universalui
//  crate rather than this crate directly.
//
//  src/geometry.rs provides some core geometric types such as uSize, uRect etc

#![allow(non_camel_case_types)]

//  uPoint - a basic location in 2D space
pub struct uPoint {
    pub x: f32,
    pub y: f32,
}

impl uPoint {

    //  standard init function from components
    pub fn init(x: f32, y: f32) -> Self {
        return uPoint {
            x: x,
            y: y
        }
    }
}

//  uSize - a basic 2D size
pub struct uSize {
    pub width: f32,
    pub height: f32,
}

impl uSize {

    //  standard init function from components
    pub fn init(width: f32, height: f32) -> Self {
        return uSize {
            width: width,
            height: height
        }
    }
}

//  uRect - a basic 2D rectangle
pub struct uRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl uRect {

    //  standard init function from components
    pub fn init(x: f32, y: f32, width: f32, height: f32) -> Self {
        return uRect {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }

}

