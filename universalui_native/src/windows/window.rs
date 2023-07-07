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
//use universalui_core::window_delegate::*;
use universalui_core::debug::*;
use universalui_core::window_provider::*;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use raw_window_handle::*;
use std::ffi::c_void;


pub struct NativeWindowProvider {

}

impl uWindowProvider for NativeWindowProvider {
    //  create window and update window handle
    fn create_window(&self, window: &uWindow) -> uWindowHandle {
        fn get_instance() -> Result<HMODULE> {
            unsafe {
                let instance = GetModuleHandleA(None)?;
                return Ok(instance);
            }
        }
    
        let instance: Option<HMODULE> = match get_instance() {
            Ok(inst) => Some(inst),
            Err(_) => {
                debug_critical("couldn't get instance handle!"); 
                None
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
                instance.unwrap(),
                None,
            );
    
            //  IMPORTANT NOTE: multiple events such as window creation are called at this stage, 
            //  before the delegate pointer is assigned. Calling a method on the delegate before
            //  it's pointer has been assigned will naturally lead to a crash.
    
            //SetWindowLongPtrW(win32_window, GWL_USERDATA, delegate as isize);
    
            let mut window_handle = Win32WindowHandle::empty();
            window_handle.hwnd = win32_window.0 as *mut c_void;
            window_handle.hinstance = instance.unwrap().0 as *mut c_void;
    
            ShowWindow(win32_window, SW_SHOW);

            return uWindowHandle{ raw_handle: Some(RawWindowHandle::from(window_handle))};
    
        }
    }
}

//  returns true if handles are equal, false if not
pub fn compare_window_handles(a: &RawWindowHandle, b: &RawWindowHandle) -> bool {

    if let RawWindowHandle::Win32(raw_a) = a {
        if let RawWindowHandle::Win32(raw_b) = b {
            if raw_a.hwnd == raw_b.hwnd {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }

}

//  returns an empty raw window handle
pub fn default_window_handle() -> RawWindowHandle {
    return RawWindowHandle::from(Win32WindowHandle::empty());
}

    

