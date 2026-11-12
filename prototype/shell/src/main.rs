//! UNNAMED OS native shell proof-of-concept (Stage 2).
//! This is a launcher window prototype, not compositor-owned shell yet.
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Grid, Label, Orientation, Box as GtkBox};

fn main() {
    let app = Application::builder().application_id("org.unnamedos.ShellPrototype").build();
    app.connect_activate(|app| {
        let root = GtkBox::new(Orientation::Vertical, 14);
        root.set_margin_top(24); root.set_margin_bottom(24);
        root.set_margin_start(24); root.set_margin_end(24);
        let title = Label::new(Some("UNNAMED OS"));
        title.add_css_class("title-1");
        let subtitle = Label::new(Some("Lightweight at heart. Beautiful by design."));
        subtitle.add_css_class("dim-label");
        let search = Entry::builder().placeholder_text("Search apps…").build();
        let grid = Grid::builder().row_spacing(10).column_spacing(10).build();
        let names = ["Files", "Terminal", "Web", "Settings", "Software", "Calculator", "Notes", "Monitor"];
        for (i, name) in names.iter().enumerate() {
            let button = Button::with_label(name);
            button.set_hexpand(true); button.set_size_request(110, 64);
            let app_name = (*name).to_string();
            button.connect_clicked(move |b| b.set_label(&format!("{} (demo)", app_name)));
            grid.attach(&button, (i % 4) as i32, (i / 4) as i32, 1, 1);
        }
        let status = Label::new(Some("Prototype only · app actions are placeholders"));
        status.add_css_class("dim-label");
        root.append(&title); root.append(&subtitle); root.append(&search);
        root.append(&grid); root.append(&status);
        let window = ApplicationWindow::builder().application(app)
            .title("UNNAMED OS Shell Prototype").default_width(720)
            .default_height(520).child(&root).build();
        window.present();
    });
    app.run();
}
