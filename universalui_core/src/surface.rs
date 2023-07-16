//  universalui_core crate - src/surface.rs
//  created by sebhall on 16/07/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/surface.rs defines the surface struct

#![allow(non_camel_case_types)]

use std::sync::Arc;
use std::vec::Vec;

pub struct uSurface {
    gpu_surface: Arc<C>
}