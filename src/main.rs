use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.Xethium.Apptrium")
        .build();

    app.connect_activate(|app| {
        // Create a window
        let setting = gtk4::Settings::default();
        //setting.set_property_gtk_theme("adwaita");
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1000)
            .default_height(600)
            .resizable(false)
            .title("Apptrium")
            .build();

        // Show the window. 
        window.present();
    });

    app.run()
}
