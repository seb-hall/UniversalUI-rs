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

use universalui_core::window_event::*;
use universalui_core::window_delegate::*;

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
            delegate.event_occurred(make_handle(window), event);
        }

        match message {
            WM_SIZE => {
                println!("WM_SIZE");
                send_event(window, uWindowEvent::resized);
            },
            _ => { }
        }
        return DefWindowProcA(window, message, wparam, lparam);
    }
}
