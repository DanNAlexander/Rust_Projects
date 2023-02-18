use gtk::prelude::*;
use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Book Catalog");
    window.set_default_size(400, 300);

    let header = gtk::HeaderBar::new();
    header.set_title("Book Catalog");
    header.set_subtitle("A simple application to catalog books");
    header.set_show_close_button(true);
    window.set_titlebar(Some(&header));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let title_entry = gtk::Entry::new();
    let author_entry = gtk::Entry::new();
    vbox.pack_start(&title_entry, false, false, 0);
    vbox.pack_start(&author_entry, false, false, 0);

    let add_button = gtk::Button::new_with_label("Add");
    vbox.pack_start(&add_button, false, false, 0);

    let books = HashMap::new();
    let list_box = gtk::ListBox::new();
    vbox.pack_start(&list_box, true, true, 0);

    window.add(&vbox);
    window.show_all();

    add_button.connect_clicked(move |_| {
        let title = title_entry.get_text().unwrap().to_owned();
        let author = author_entry.get_text().unwrap().to_owned();
        let book = Book { title, author };
        books.insert(title.clone(), book);

        let row = gtk::ListBoxRow::new();
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let label = gtk::Label::new(Some(&format!("{} by {}", title, author)));
        hbox.pack_start(&label, true, true, 0);
        row.add(&hbox);
        list_box.insert(&row, -1);
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
