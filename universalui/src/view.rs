//  universalui crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/view.rs defines the view trait and the view handler

#![allow(non_camel_case_types)]

use std::sync::Arc;
use std::vec::Vec;


pub trait uView {
    fn init(&self);
    fn draw(&self);
    fn add_subview(&self);
    fn subviews(&self) -> Vec<Arc<dyn uView>>;
}

pub struct uViewHandler {
    pub root_view: Arc<dyn uView>,
    pub view_did_load: fn(),
}