//  universalui_native crate - src/windows/loop.rs
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
//  src/windows/loop.rs contains the windows implementation 
//  of the main event loop.

use crate::native::window::*;

use universalui_core::geometry::*;
use universalui_core::debug::*;
use universalui_core::window::*;
//use universalui_core::window_event::*;
//use universalui_core::window_delegate::*;

use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use raw_window_handle::*;
use std::ffi::c_void;

//static window_delegate: *mut uWindowDelegate = std::ptr::null();

pub fn run() {
    let mut msg = MSG::default();
                
    unsafe {
        while GetMessageW(&mut msg, HWND(0), 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    
}

pub unsafe extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT { 

    //println!("an event occured! {}", message);
    
    unsafe {
        /* 
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
        
        unsafe fn decode_delegate(window: HWND) -> *mut uWindowDelegate {
            let delegate_ptr: *mut uWindowDelegate = GetWindowLongPtrW(window, GWL_USERDATA) as *mut uWindowDelegate;
            return delegate_ptr;
            
        }

        unsafe fn send_event(window: HWND, event: uWindowEvent) {
            let delegate = &mut *decode_delegate(window);
            let window_handle = make_handle(window);

            if delegate.is_multiple_windows() {
                let delegate_ref = &mut *decode_delegate(window);
                let windows_vec: &mut Vec<uWindow> = delegate_ref.get_windows().unwrap();

                for window_ref in windows_vec {
                    if let Some(simple_window_handle) = window_ref.raw_handle { // break if handle not defined for window
                        if compare_window_handles(&simple_window_handle, &window_handle) { // window defined and the one the event happened to
                            delegate.event_occurred(window_ref, event);
                            return;
                        }
                    }
                }

                debug_warning("a window event occured for an unknown window!");

            } else {
                let delegate_ref = &mut *decode_delegate(window);
                let simple_window: &mut uWindow = delegate_ref.get_window().unwrap().as_mut().unwrap();

                if let Some(simple_window_handle) = simple_window.raw_handle { // break if handle not defined for window
                    if compare_window_handles(&simple_window_handle, &window_handle) { // simple window defined and the one the event happened to
                        delegate.event_occurred(simple_window, event);
                    } else { // simple window defined but not the one from the event
                        debug_warning("a window event occured for an unknown window!");
                    }
                }
            }
            
            if let Some(has_simple_window) = delegate.get_window() { // break if not a simple window
                if let Some(simple_window) = has_simple_window { // break if window not defined
                    if let Some(simple_window_handle) = simple_window.raw_handle { // break if handle not defined for window
                        if compare_window_handles(&simple_window_handle, &window_handle) { // simple window defined and the one the event happened to
                            
                        } else { // simple window defined but not the one from the event

                        }
                    }

                    

                }

                

                
            }

        }

        match message {
            WM_SIZE => {
                let width = loword(lparam.0 as u32);
                let height = hiword(lparam.0 as u32);
                send_event(window, uWindowEvent::resized{to_size: uSize::init(width as f32, height as f32)});
            },
            _ => { }
        }*/
        return DefWindowProcA(window, message, wparam, lparam);
    }
}
