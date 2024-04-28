extern crate gtk;
extern crate gio;

use gtk::prelude::*;

use gtk::{Builder, Window, Button, FileChooserDialog, TreeView, TreeStore, TreeViewColumn, CellRendererText};

use std::env::args;
use std::fs::File;
use arrow_array::StringArray;
use arrow_schema::DataType;
use gdk::gio::ListStore;
use gio::ListStoreBuilder;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

// the handler
fn on_clicked(param: &[glib::Value]) -> Option<glib::Value> {
    println!("on_start_clicked fired!");
    None
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../resources/gtk4test.ui");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.object("window1").expect("Couldn't get window");

    let file_chooser: FileChooserDialog = builder.object("fileChooser1").expect("No chooser");
    file_chooser.set_application(Some(application));

    let tree: TreeView = builder.object("treeView1").expect("No TreeView");
    file_chooser.connect_response(move |chooser, resp_type| {
        let file = chooser.file().unwrap();
        let path = file.path().unwrap();
        println!("file={file:?} resp_type={resp_type:?}");

        let file = File::open(path).unwrap();

        let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
        println!("Converted arrow schema is: {}", builder.schema().fields.len());

        let mut reader = builder.build().unwrap();

        let batch = reader.next().unwrap().unwrap();
        println!("batch has {} rows", batch.num_rows());

        let types = batch.schema().fields().iter().map(|f| String::static_type()).collect::<Vec<_>>();
        let store = TreeStore::new(types.as_slice());
        tree.set_model(Some(&store));
        for (idx, f) in batch.schema().fields.iter().enumerate() {
            let column = TreeViewColumn::new();
            let cell = CellRendererText::new();
            column.set_title(f.name().as_str());
            column.pack_start(&cell, true);
            column.add_attribute(&cell, "text", idx as i32);
            tree.append_column(&column);
        }
        tree.set_headers_visible(true);

        let row_iters = (0..batch.num_rows()).map(|_| store.append(None)).collect::<Vec<_>>();
        let data = batch.columns().iter().enumerate().map(|(col_idx, col)| {
            let vals = match col.data_type() {
                DataType::Utf8 => {
                    let col = col.as_any().downcast_ref::<StringArray>().unwrap();
                    let vals = row_iters.iter().enumerate().map(|(row_idx, iter)| {
                        let val = col.value(row_idx).to_value();
                        store.set_value(&iter, col_idx as u32, &val);
                        val
                    }).collect::<Vec<_>>();
                    vals
                }
                _ => vec![],
            };
            vals
        }).collect::<Vec<_>>();

        chooser.hide();
    });

    let btn: Button = builder.object("btnFileOpen").expect("Cant get button");
    btn.connect_clicked(move |_| {
        file_chooser.show();
        println!("Activated");
    });

    window.set_application(Some(application));

    // window.connect_delete_event(|_, _| {
    //     gtk::main_quit();
    //     Inhibit(true)
    // });

    // builder.connect_signals(|builder, handler_name| {
    //     match handler_name {
    //         // handler_name as defined in the glade file => handler function as defined above
    //         "_on_clicked" => Box::new(on_clicked),
    //         _ => Box::new(|_| {None})
    //     }
    // });

    window.show();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.test.app"),
        Default::default(),
    );

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}
