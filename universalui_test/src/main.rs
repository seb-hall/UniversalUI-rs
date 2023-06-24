use universalui::*;

fn resized(_: core::geometry::uSize) {
    println!("window resized!");
}

fn main() {
    universalui_init();

    let mut delegate = core::window::uWindowDelegate { 
        window_resized: resized,
        ..Default::default()
    };

    delegate.window_resized = resized;

    let my_string = core::string::uString::init("here is some string");

    let window = core::window::uWindow::init(my_string, core::geometry::uRect::init(0.0, 0.0, 100.0, 100.0), delegate);

    (window.delegate.window_resized)(core::geometry::uSize::init(100.0, 100.0));

    println!("testing universalui");
}
