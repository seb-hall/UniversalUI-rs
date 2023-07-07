use universalui::{*, core::{string::*, window::*, geometry::*, view::*, application::*, debug::*}};


fn finished_launch(app: &mut uDesktopApplication) {

    println!("hi from test finished launch!");

    let myWindow = uWindow::init("I am window", uSize::init(800.0, 600.0));
    app.add_window(myWindow);

    let myWindow2 = uWindow::init("I am window 2", uSize::init(800.0, 600.0));
    app.add_window(myWindow2);
}


fn main() {

    let mut app = uDesktopApplication::init( "my app", "me", 0, 1 );
    app.set_finished_launching_callback(finished_launch);

    universalui_init(&mut app);
}