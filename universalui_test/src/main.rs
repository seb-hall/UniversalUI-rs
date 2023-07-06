use universalui::{*, application_window::*, core::{string::*, window::*, geometry::*, view::*, application::*}};


fn finished_launch(sender: &mut uApplication) {

    /*let mut window = uApplicationWindow{
        title: uString::init("test window"),
        size: uSize::init(800.0, 600.0),
        root: uTestView { }
    };

    
    //let mut window1 = uWindow::init(uString::init("Test Window 1"), uSize::init(800.0, 600.0));
    //let mut window2 = uWindow::init(uString::init("Test Window 2"), uSize::init(800.0, 600.0));

    sender.show_window(window1);
    sender.show_window(window2);
    */
}

fn will_q() {

}

fn main() {

    let app = uApplication {
        name: uString::init("my app"),
        developer: uString::init("me"),
        major_version: 0,
        minor_version: 1,
        finished_launching: will_q,
        will_quit: will_q
    };

    universalui_init(app);
}