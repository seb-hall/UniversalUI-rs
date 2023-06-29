use universalui::*;

fn resized(_: geometry::uSize) {
    println!("window resized!");
}

fn main() {
    universalui_init();

    let mut handler = windowHandler::uWindowHandler { 
        window_resized: resized,
        ..Default::default()
    };

    handler.window_resized = resized;

    let my_string = string::uString::init("here is some string");

    let window = window::uWindow::init(my_string, geometry::uRect::init(0.0, 0.0, 100.0, 100.0), handler);
    
    (window.handler.window_resized)(geometry::uSize::init(100.0, 100.0));
    (window.handler.window_created)();

    println!("testing universalui");

    while (true) {

    }
}