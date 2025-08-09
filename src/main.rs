use gtk::prelude::*;
use gtk::{glib, Application};

mod windows;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create and show the main window using the new structure
    let main_window = windows::MainWindow::new(app);
    main_window.present();
}