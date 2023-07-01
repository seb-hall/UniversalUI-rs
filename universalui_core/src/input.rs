//  universalui crate - src/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/input.rs defines the input event, the input reciever trait and the input handler

#![allow(non_camel_case_types)]

pub enum uInputEventType {
    normalKey,
    escapeKey,
    controlKey, // is this right - does it get called at key down or up??
    spaceKey,
    tabKey,
    backspaceKey,
    deleteKey,
    arrowUpKey,
    arrowDownKey,
    arrowLeftKey,
    arrorRightKey
}

pub trait uInputEventReceiver {
    
}