//  universalui_native crate - src/windows/mod.rs
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
//  src/windows/mod.rs contains the root of the windows
//  specific platform module.

#![allow(non_camel_case_types)]

pub mod window;

use universalui_core::debug::*;

use universalui_core::{
    window::*,
    window_provider::*,
};

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use raw_window_handle::*;
use std::os::raw::c_void;

pub struct uNativeWindowProvider {
    pub raw_ptr: Option<isize>
}

impl uWindowProvider for uNativeWindowProvider {
    fn create_window(&self, window: &uWindow) -> uWindowHandle { 
        return window::create_window(window, self.raw_ptr.unwrap());
    }

    fn run_event_loop(&self) {
        let mut msg = MSG::default();
                    
        unsafe {
            while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    //  init function, register window class
    fn init(&mut self) -> bool {

        debug_info("Initialising UniversalUI for Windows...");

        fn get_instance() -> Result<HMODULE> {
            unsafe {
                let instance = GetModuleHandleA(None)?;
                return Ok(instance);
            }
        }

        fn create_class(instance: HMODULE) -> Result<WNDCLASSA> {
            unsafe {
                let window_class = s!("window");
                
                let wc = WNDCLASSA {
                    hCursor: LoadCursorW(None, IDC_ARROW)?,
                    hInstance: instance,
                    lpszClassName: window_class,
                    style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
                    lpfnWndProc: Some(wndproc),
                    ..Default::default()
                };

                return Ok(wc);
            }
        }

        fn register_class(wc: WNDCLASSA) -> Result<bool> {
            unsafe {
                let atom = RegisterClassA(&wc);
                if atom == 0 {
                    return Ok(false);
                }

                return Ok(true);
            }
        }


        let instance = match get_instance() {
            Ok(inst) => inst,
            Err(_) => {
                debug_critical("couldn't get instance handle!"); 
                return false; 
            }
        };

        if instance.0 == 0 {
            return false;
        }

        let class = match create_class(instance) {
            Ok(wc) => wc,
            Err(_) => {
                debug_critical("couldn't create a window class!"); 
                return false; 
            }
        };


        let _ = match register_class(class) {
            Ok(result) => if !result { return false; }
            Err(_) => {
                debug_critical("couldn't register class!"); 
                return false; 
            }
        };

        let raw: *mut uNativeWindowProvider = self;
        self.raw_ptr = Some(raw as isize);

        return true;
    }
      
}

unsafe extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT { 

    //println!("an event occured! {}", message);
    
    unsafe {
        
        unsafe fn loword(x: u32) -> u16 {
            (x & 0xFFFF) as u16
        }

        unsafe fn hiword(x: u32) -> u16 {
            ((x >> 16) & 0xFFFF) as u16
        }

        unsafe fn make_handle(window: HWND) -> RawWindowHandle {
            let mut window_handle = Win32WindowHandle::empty();
            window_handle.hwnd = window.0 as *mut c_void;
            window_handle.hinstance = GetWindowLongA(window, GWL_HINSTANCE) as *mut c_void;

            return RawWindowHandle::from(window_handle);
        }
        
        unsafe fn decode_provider(window: HWND) -> *mut uNativeWindowProvider {
            let provider_ptr: *mut uNativeWindowProvider = GetWindowLongPtrW(window, GWL_USERDATA) as *mut uNativeWindowProvider;
            return provider_ptr;
        }


        match message {
            WM_SIZE => {
                let width = loword(lparam.0 as u32);
                let height = hiword(lparam.0 as u32);
                debug_info(&format!("Window Resized: {} {}", width as f32, height as f32)[..]);
            },
            _ => { }
        }
        return DefWindowProcA(window, message, wparam, lparam);
    }
}






