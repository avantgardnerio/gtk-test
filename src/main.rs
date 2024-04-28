extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Builder, Window, Button, MenuItem, FileChooserDialog};

use std::env::args;
use std::fs::File;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

// the handler
fn on_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    println!("on_start_clicked fired!");
    None
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../resources/example.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.get_object("window1").expect("Couldn't get window");

    let file_chooser: FileChooserDialog = builder.get_object("fileChooser1").expect("No chooser");
    file_chooser.set_application(Some(application));

    file_chooser.connect_file_activated(|chooser| {
        let file = chooser.get_file().unwrap();
        let path = file.get_path().unwrap();
        println!("file={file:?}");

        let file = File::open(path).unwrap();

        let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
        println!("Converted arrow schema is: {}", builder.schema());

        let mut reader = builder.build().unwrap();

        let record_batch = reader.next().unwrap().unwrap();
        println!("batch has {} rows", record_batch.num_rows());

        chooser.hide();
    });

    let btn: MenuItem = builder.get_object("button1").expect("Cant get button");
    btn.connect_activate(move |_| {
        file_chooser.show_all();
        println!("Activated");
    });

    window.set_application(Some(application));
    window.set_title("Test");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(true)
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
