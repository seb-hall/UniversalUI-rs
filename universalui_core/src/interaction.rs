//  universalui_core crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/interaction.rs defines the pointer event, the pointer reciever trait and the pointer handler

#![allow(non_camel_case_types)]

use crate::geometry::*;
use crate::view::*;

pub enum uPointerEventType {
    pointerMove,            //  pointer move in reciever e.g mouse moved
    pointerEnter,           //  pointer enter reciever, e.g drag and drop
    pointerExit,            //  pointer leave reciever, e.g drag and drop
    pointerDown,            //  pointer down, e.g mouse button pressed down or finger pressed down
    pointerUp,              //  pointer up, e.g mouse button released or finger lifted
    pointerDownEnter,       //  
    pointerDownExit         
}

pub struct uPointerEvent {
    pub event_type: uPointerEventType,
    pub location: uPoint,
}

pub trait uPointerReciever: uView {
    // reciever rect
    // functions?
}

pub struct uPointerHandler {
    // functions? inherit from 
}
