use universalui::{*, application::uApplication, core::string::uString, core::{window::uWindow, geometry::*}};

fn finished_launch(sender: &mut uApplication) {

    let window = uWindow::init(uString::init("hello universalui"), uRect::init(0.0, 0.0, 800.0, 600.0));
    sender.show_window(window);
}

fn will_q() {

}

fn main() {
    
    let mut app = uApplication::init_desktop(
        "universalui test", 
        "developer", 
        0, 
        1,
        finished_launch,
        will_q
    );

    universalui_init(&mut app);
    
}