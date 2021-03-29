extern crate gtk;
// extern crate glib;

use gtk::prelude::*;
// use gtk::BuilderExt;
// use gtk::{ButtonExt};
// use glib::object::ObjectExt;
// use glib::ObjectExt::

pub fn run(){
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("tutorial.glade");
    let builder = gtk::Builder::from_string(glade_src);
    
    let window: gtk::Window = builder.get_object("window1").unwrap();
    let button: gtk::Button = builder.get_object("button1").unwrap();
    let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();
    
    button.connect_clicked(move |_| {
        dialog.run();
        dialog.hide();
    });
    
    window.show_all();
    
    gtk::main();
}

pub fn run2() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("tutorial2.glade");
    let builder = gtk::Builder::from_string(glade_src);
    
    let window: gtk::Window = builder.get_object("Main_window").unwrap();

    // let button: gtk::Button = builder.get_object("Button_top").unwrap();
    // let dialog: gtk::MessageDialog = builder.get_object("messagedialog1").unwrap();
    
    // button.connect_clicked(move |_| {
    //     dialog.run();
    //     dialog.hide();
    // });
    
    std::mem::drop(builder);
    window.show_all();
    
    gtk::main();
}

pub fn run3(){
    if gtk::init().is_err(){
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("attempt.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("Main_window").unwrap();

    window.connect_delete_event(|_, _| { gtk::main_quit(); Inhibit(false) });
    
    std::mem::drop(builder);
    window.show_all();

    gtk::main();


}