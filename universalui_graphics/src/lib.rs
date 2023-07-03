pub mod surface;
pub mod text;

use universalui_core::debug::*;

pub enum uGraphicsApi {
    OpenGL,
    Vulkan,
    Metal
}

//  init universalui_graphics
pub fn init(api: uGraphicsApi) {
    match api {
        uGraphicsApi::OpenGL => {

        },

        uGraphicsApi::Vulkan => {

        },

        uGraphicsApi::Metal => {
            cfg_if::cfg_if! {
                if #[cfg(windows)] {
                    debug_critical("Windows requested Vulkan! This is not OK!");
                } 
            }
        }
    }
}