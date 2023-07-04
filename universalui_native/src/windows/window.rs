//  universalui_native crate - src/windows/window.rs
//  created by sebhall on 04/07/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  The universalui_native crate communicates directly with
//  native platform APIs such as Win32 and Cocoa to provide
//  abstracted, cross platform such as windowing and file 
//  management, without relying on external dependancies.
//
//  src/windows/window.rs contains functions for creating#
//  and managing top-level windows.

use universalui_core::window::*;
use universalui_core::debug::*;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

//  basic init funciton from a frame and title
pub fn create_window(window: uWindow) -> bool {

    fn get_instance() -> Result<HMODULE> {
        unsafe {
            let instance = GetModuleHandleA(None)?;
            return Ok(instance);
        }
    }

    let instance = match get_instance() {
        Ok(inst) => inst,
        Err(_) => {
            debug_critical("couldn't get instance handle!"); 
            return false; 
        }
    };

    unsafe {
        let _window: HWND = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("window"),
            PCSTR(window.title.str().as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window.frame.width as i32,
            window.frame.height as i32,
            None,
            None,
            instance,
            None,
        );
    }

    return true;

}

    

    
