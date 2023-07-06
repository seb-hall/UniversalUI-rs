use universalui::{*, application::*, core::{string::*, window::*, geometry::*}};


fn finished_launch(sender: &mut uApplication) {

    let mut window1 = uWindow::init(uString::init("Test Window 1"), uSize::init(800.0, 600.0));
    let mut window2 = uWindow::init(uString::init("Test Window 2"), uSize::init(800.0, 600.0));

    sender.show_window(window1);
    sender.show_window(window2);

}

fn will_q() {

}

fn main() {
    
    let app = uApplication::init_desktop(
        "universalui test", 
        "developer", 
        0, 
        1,
        //uRect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        finished_launch,
        will_q
    );

    universalui_init(app);
}