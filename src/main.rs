extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Builder, Window, Button, MenuItem, FileChooserDialog, TreeView, TreeStore, TreeViewColumn, CellRendererText};

use std::env::args;
use std::fs::File;
use gdk::gio::ListStore;
use gio::ListStoreBuilder;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

// the handler
fn on_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    println!("on_start_clicked fired!");
    None
}

fn append_text_column(tree: &TreeView) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.set_title("Text");
    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
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

    let left_tree: TreeView = builder.get_object("treeView1").expect("No TreeView");
    let left_store = TreeStore::new(&[String::static_type()]);
    left_tree.set_model(Some(&left_store));
    append_text_column(&left_tree);
    left_tree.set_headers_visible(true);

    for i in 0..10 {
        // insert_with_values takes two slices: column indices and ToValue
        // trait objects. ToValue is implemented for strings, numeric types,
        // bool and Object descendants
        let _ = left_store.insert_with_values(None, None, &[0], &[&format!("Hello {}", i)]);
    }

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
