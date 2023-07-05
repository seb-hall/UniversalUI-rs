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
use universalui_core::window_delegate::*;
use universalui_core::debug::*;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use raw_window_handle::*;
use std::ffi::c_void;

//  basic init funciton from a frame and title
pub fn create_window(window: &mut uWindow, delegate: *mut uWindowDelegate) -> bool {

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
        let win32_window: HWND = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            s!("window"),
            PCSTR(window.title.str().as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window.size.width as i32,
            window.size.height as i32,
            None,
            None,
            instance,
            None,
        );

        println!("got to here!");

        SetWindowLongPtrW(win32_window, GWL_USERDATA, delegate as isize);

        let mut window_handle = Win32WindowHandle::empty();
        window_handle.hwnd = win32_window.0 as *mut c_void;
        window_handle.hinstance = instance.0 as *mut c_void;

        window.raw_handle = Some(RawWindowHandle::from(window_handle));

        ShowWindow(win32_window, SW_SHOW);

    }
    
    return true;

}

    

    
