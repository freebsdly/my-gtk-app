use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Label, Box as GtkBox};

pub struct ToolsWindow {
    window: ApplicationWindow,
}

impl ToolsWindow {
    pub fn new() -> Self {
        let window = ApplicationWindow::builder()
            .title("Tools")
            .default_width(400)
            .default_height(300)
            .build();

        let content = GtkBox::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .build();

        let label = Label::builder()
            .label("This is the Tools window")
            .css_classes(["heading"])
            .build();

        let close_button = Button::builder()
            .label("Close")
            .build();
        
        let close_window = window.clone();
        close_button.connect_clicked(move |_| {
            close_window.close();
        });

        content.append(&label);
        content.append(&close_button);
        
        window.set_child(Some(&content));
        
        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}