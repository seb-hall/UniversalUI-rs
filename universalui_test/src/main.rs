use universalui::{*, application::uApplication, core::string::uString};

fn finished_launch() {

}

fn will_q() {

}

fn main() {
    
    let app = uApplication::init_simple(
        "universalui test", 
        "developer", 
        0, 
        1,
        finished_launch,
        will_q
    );

    let num_windows = app.windows().len();

    universalui_init(app);

    

}