#![allow(non_camel_case_types)]

pub mod surface;
pub mod text;

use std::collections::HashMap;

use raw_window_handle::*;

use universalui_core::{
    debug::*,
    window::*
};

use wgpu::*;

pub struct uGraphicsWindow {

}

pub struct uGraphicsProvider {
    instance: Instance,
    window_map: HashMap<RawWindowHandle, uGraphicsWindow>
}

impl uGraphicsProvider {

    //  init universalui_graphics
    pub fn init() -> Self {

        debug_info("initialising UniversalUI-Graphics");

        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        return uGraphicsProvider {
            instance: instance,
            window_map: HashMap::new()
        };

    }

    pub fn setup_for_window(&self, window: &uWindow, handle: &uWindowHandle) -> bool {

        let surface = match unsafe { self.instance.create_surface(&handle) } {
            Ok(surface) => {debug_info("surface ok"); surface}
            Err(_) => {
                debug_info("couldnt create surface");
                panic!()
            }
        };

        debug_info("got surface");

        let adapter: Adapter = pollster::block_on( 
            async { 
                self.instance.request_adapter(
                    &wgpu::RequestAdapterOptions {
                        power_preference: wgpu::PowerPreference::default(),
                        compatible_surface: Some(&surface),
                        force_fallback_adapter: false,
                    },
                ).await.unwrap() 
            } 
        );

        let (device, queue): (Device, Queue) = pollster::block_on(
            async { 
                adapter.request_device(
                    &wgpu::DeviceDescriptor {
                        features: wgpu::Features::empty(),
                        limits: wgpu::Limits::default(),
                        label: None,
                    },
                    None, // Trace path
                ).await.unwrap()
            }
        );

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.size.width as u32,
            height: window.size.height as u32,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let window_stuff = uGraphicsWindow {

        };

        debug_error("jk everything ok");
        
        return true;
    } 

}