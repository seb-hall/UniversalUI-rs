//  universalui_core crate - src/window_event.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/window_event.rs defines the window event enum

#![allow(non_camel_case_types)]

use crate::geometry::*;

pub enum uWindowEvent {
    resized {to_size: uSize},
    shown,
    hidden
}