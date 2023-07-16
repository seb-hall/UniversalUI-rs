#![allow(non_camel_case_types)]

pub mod surface;
pub mod text;

use std::collections::HashMap;

use raw_window_handle::*;

use universalui_core::{
    geometry::*,
    debug::*,
    window::*
};

use wgpu::*;

pub struct uGraphicsWindow {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: uSize,
    render_pipeline: wgpu::RenderPipeline,
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

    pub fn setup_for_window(&mut self, window: &uWindow, handle: &uWindowHandle) -> bool {

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

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let window_stuff = uGraphicsWindow {
            surface,
            device,
            queue,
            config,
            size: uSize::init(window.size.width, window.size.height),
            render_pipeline,
        };

        self.window_map.insert(handle.raw_window_handle(), window_stuff);

        debug_error("jk everything ok");
        
        return true;
    } 

}