//  universalui - src/win32/window.rs
//  created by sebhall on 24/06/2023
//
//  UniversalUI is a cross-platform application development
//  framework. Placing high priority on efficiency and
//  ease of use, UniversalUI is suitable for a wide variety
//  of applications, from simple GUI tools to full-fledged
//  creative applications and games.
//
//  src/win32/lib.rs contains win32-specific window functionality
//  for UniversalUI.


#![allow(non_camel_case_types)]

use crate::windowHandler::uWindowHandler;
use crate::geometry::*;

use std::sync::mpsc::{channel, Sender};
use std::vec::Vec;

use std::rc::Rc;

use std::thread::{self, JoinHandle};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::{core::*, s};
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Graphics::Gdi::*;

pub fn NewWindow(width: f32, height: f32, title: String, handler: Rc<uWindowHandler>) {
    let mut newWindow = w32Window::init(width as i32, height as i32, title);
    newWindow.handler = handler;
    unsafe { 
        windows.push(newWindow);
    }
}


static mut windows: Vec<w32Window> = Vec::new();


//  w32Window is a struct for Win32 windows specifically
pub struct w32Window {
    pub w32Handle: HWND,
    pub handler: Rc<uWindowHandler>
}

impl w32Window {

    //  basic init funciton from a frame and title
    pub fn init(width: i32, height: i32, title: String) -> w32Window {

        fn thread_init(width: i32, height: i32, title: String) {

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
                    width,
                    height,
                    None,
                    None,
                    instance,
                    None,
                );

        
                let mut message = MSG::default();
                

                let mut should_quit: bool = false;

                let mut client_rect = RECT::default();

                while !should_quit {
                    if bool::from(PeekMessageA(&mut message, window, 0, 0, PM_REMOVE)) {
                        TranslateMessage(&message);
                        DispatchMessageA(&message);
                
                        /*(match message.message {
                            WM_QUIT => {
                                println!("QUIT");
                                should_quit = true;
                            }
                            WM_PAINT => {
                                println!("PAINT");
                                // Perform painting/update logic here
                                RedrawWindow(window, None, None, RDW_VALIDATE);
                            }
                            WM_SIZE => {
                                println!("RESIZE");
                                InvalidateRect(window, None, true);
                            }
                            WM_MOVE => {
                                println!("MOVE");
                                InvalidateRect(window, None, true);
                            }
                            WM_MOUSEMOVE => {
                                println!("MOUSEMOVE");
                            }
                            WM_POSCHANGED => {
                                println!("POSCHANGED");
                            }
                            _ => {
                                // Other message handling logic
                                println!("{}", message.message);
                            }
                        }*/
                    } else {
                        // Perform other non-message related work here
                    }
                }

            };

        }

        thread::spawn(move || {
            thread_init(width, height, title);
        });
        
        return w32Window {
            w32Handle: HWND::default(),
            handler: Rc::new(uWindowHandler::default())
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
        match message {
            
            WM_PAINT => {
                println!("WM_PAINT");
                

                let hdc =  GetDC(window);

                let mut rect: RECT = RECT { ..Default::default() };
                GetClientRect(window, &mut rect);

                /*glViewport(0, 0, rect.right, rect.bottom);
                glMatrixMode(GL_PROJECTION);
                glLoadIdentity();
                gluPerspective(70.0, rect.right as f64/rect.bottom as f64, 0.0, 1000.0);
                glMatrixMode(GL_MODELVIEW);
                glLoadIdentity();
                gluLookAt(0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
                glClearColor(0.2, 0.2, 0.2, 0.2);
                glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);


                glBegin(GL_TRIANGLES);
                glColor3f(1.0, 0.0, 0.0);
                glVertex3f(-5.0, -4.0, 0.0);
                glColor3f(0.0, 1.0, 0.0);
                glVertex3f(5.0, -4.0, 0.0);
                glColor3f(0.0, 0.0, 1.0);
                glVertex3f(0.0, 3.5, 0.0);
                glEnd();

                SwapBuffers(hdc);*/

                let mut ps: PAINTSTRUCT = PAINTSTRUCT { ..Default::default() };
                let hdc = BeginPaint(window, &mut ps);
               
                //let mut rect: RECT = RECT { ..Default::default() };
                //GetClientRect(window, &mut rect);
                let message = format!("Hello, Windows! {}, {}", rect.right, rect.bottom);
                TextOutA(hdc, 0, 0, &message.as_bytes());
                EndPaint(window, &mut ps);

                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_SIZE => {
                println!{"WM_SIZE"}

                let mut rect: RECT = RECT { ..Default::default() };
                GetClientRect(window, &mut rect);
                for windowStruct in windows.iter() {
                    (windowStruct.handler.window_resized)(uSize::init(rect.right as f32, rect.bottom as f32));
                    
                }
                
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam)
        }
    }
}

