use universalui::*;

fn resized(_: geometry::uSize) {
    println!("window resized!");
}

fn main() {
    universalui_init();

    let mut handler = window::uWindowHandler { 
        window_resized: resized,
        ..Default::default()
    };

    handler.window_resized = resized;

    let my_string = string::uString::init("here is some string");

    //let window = window::uWindow::init(my_string, geometry::uRect::init(0.0, 0.0, 1000.0, 750.0), handler.clone());
    let window2 = window::uWindow::init(string::uString::init("here is sanother string with some text"), geometry::uRect::init(0.0, 0.0, 1000.0, 750.0), handler);
    
    (window2.handler.window_resized)(geometry::uSize::init(100.0, 100.0));
    (window2.handler.window_created)();

    println!("testing universalui");

    while (true) {

    }
}