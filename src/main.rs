extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Builder,Window, Button};

use std::env::args;

// the handler
fn on_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    println!("on_start_clicked fired!");
    None
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../resources/example.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.get_object("window1").expect("Couldn't get window");

    window.set_application(Some(application));
    window.set_title("Test");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
    });

    // directly calling the button1 without implementing signal
    let btn: Button = builder.get_object("button1").expect("Cant get button");
    btn.connect_clicked(|_| {
    println!("Activated");
    });


    // builder.connect_signals(|builder, handler_name| {
    //     match handler_name {
    //         // handler_name as defined in the glade file => handler function as defined above
    //         "_on_clicked" => Box::new(on_clicked),
    //         _ => Box::new(|_| {None})
    //     }
    // });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.test.app"),
        Default::default(),
    )
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
