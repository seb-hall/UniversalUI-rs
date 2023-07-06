

use universalui_core::geometry::*;
use universalui_core::string::*;
use universalui_core::window::*;
use universalui_core::view::*;


pub struct uApplicationWindow {

    pub title: uString,
    pub size: uSize,
    pub root: dyn uView

}
