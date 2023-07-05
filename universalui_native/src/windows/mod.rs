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

pub mod window;
pub mod event_loop;

use universalui_core::debug::*;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

//  init function, register window class
pub fn init() -> bool {

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
                lpfnWndProc: Some(event_loop::wndproc),
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

    return true;
}




