//  universalui_core crate - src/eventloop_provider.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/eventloop_provider.rs defines the uEventloopProvider trait. 

#![allow(non_camel_case_types)]

//  to be implemented by graphics system
pub trait uEventloopProvider {
    fn init(&mut self) -> bool;
    fn run(&self) -> bool;
}