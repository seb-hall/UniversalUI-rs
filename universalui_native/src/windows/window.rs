#![allow(non_camel_case_types)]

//  window module
pub mod window {

    use std::sync::mpsc::{channel, Sender};

    use std::thread::{self, JoinHandle};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    use crate::core::*;
    use windows::{core::*, s};
    use windows::Win32::Foundation::*;
    use windows::Win32::System::LibraryLoader::GetModuleHandleA;
    use windows::Win32::UI::WindowsAndMessaging::*;


    pub enum WindowType {
        NORMAL,
        POPUP
    }

    //  w32Window is a struct for Win32 windows specifically
    pub struct w32Window {
        pub w32Handle: HWND,
        pub frame: uRect,
        pub title: String,
        pub windowType: WindowType,
        //pub joinHandle: JoinHandle<Thread>
    }

    impl w32Window {

        //  basic init funciton from a frame and title
        pub fn init(frame: uRect, title: String, windowType: WindowType) -> w32Window {

            fn thread_init(frame: uRect, title: String) {

                fn get_instance() -> Result<HMODULE> {
                    unsafe {
                        let instance = GetModuleHandleA(None)?;
                        return Ok(instance);
                    }
                }
    
                let instance = match get_instance() {
                    Ok(inst) => inst,
                    Err(error) => return
                };

                let titleStr = &title[..];
    
                unsafe {
                    let window: HWND = CreateWindowExA(
                        WINDOW_EX_STYLE::default(),
                        s!("window"),
                        PCSTR(titleStr.as_ptr()),
                        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                        CW_USEDEFAULT,
                        CW_USEDEFAULT,
                        frame.width as i32,
                        frame.height as i32,
                        None,
                        None,
                        instance,
                        None,
                    );
    
            
                    let mut msg = MSG::default();
            
                    while GetMessageW(&mut msg, window, 0, 0) != false {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                };
    
            }

            thread::spawn(move || {
                thread_init(frame, title);
            });
            
            return w32Window {
                w32Handle: HWND::default(),
                frame: uRect::init(0.0, 0.0, 100.0, 100.0),
                title: String::from("hi"),
                windowType: windowType
            }

        }

        

        
    }


    pub fn init() -> bool {

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
                if (atom == 0) {
                    return Ok(false);
                }

                return Ok(true);
            }
        }


        let instance = match get_instance() {
            Ok(inst) => inst,
            Err(_) => return false
        };

        if (instance.0 == 0) {
            return false;
        }

        let class = match create_class(instance) {
            Ok(wc) => wc,
            Err(_) => return false
        };


        let _ = match register_class(class) {
            Ok(result) => if (!result) { return false; }
            Err(_) => return false
        };

        return true;
    }

    extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT { 
        unsafe {
            DefWindowProcA(window, message, wparam, lparam)
        }
    }

}
