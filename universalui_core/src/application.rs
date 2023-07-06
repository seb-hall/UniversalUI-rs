use crate::string::*;
use crate::geometry::*;
use crate::view::*;

pub struct uApplication {
    pub name: uString,
    pub developer: uString,
    pub major_version: i32,
    pub minor_version: i32,

    pub finished_launching: fn(),
    pub will_quit: fn()
}