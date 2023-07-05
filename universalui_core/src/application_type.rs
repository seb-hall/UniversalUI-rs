//  universalui_core crate - src/application_type.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/application_type.rs defines the application type enum

#![allow(non_camel_case_types)]

//  uApplicationType struct, takes three values:
//  SIMPLE applications contain a single, automatically created window.
//  The preferred size of this window can be set for some platforms, 
//  notably macOS, Windows and most Linux configurations.
//  DESKTOP applications are recommended for desktop platforms. They can 
//  contain any number of windows, none of which are created automatically. 
//  Instead, an empty vector is initalised.
//  OTHER applications are intended for tools and applets. They lack
//  desktop features such as a menu bar.
pub enum uApplicationType {
    simple,
    desktop,
    other
}