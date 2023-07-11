#![allow(non_camel_case_types)]

pub mod surface;
pub mod text;

use universalui_core::{
    debug::*,
    window::*
};

use wgpu::*;

pub struct uGraphicsProvider {
    instance: Instance,

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
            instance
        };

    }

    pub fn setup_for_window(&self, handle: &uWindowHandle) -> bool {

        let surface = match unsafe { self.instance.create_surface(&handle) } {
            Ok(surface) => {debug_info("surface ok"); surface}
            Err(_) => {
                debug_info("couldnt create surface");
                panic!()
            }
        };

        debug_info("got surface");

        debug_error("jk everything ok");
        
        return true;
    } 

    async fn getStuff(&self, surface: Surface) { let adapter: Adapter =  self.instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap(); 

    let (device, queue): (Device, Queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web we'll have to disable some.
            limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            label: None,
        },
        None, // Trace path
    ).await.unwrap();

}

}