/// theridev was here
/// Nov 18: Main
/// Nov 19: Apptrium-Legacy-ToRust

// Imports
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

// Imports: Apptrium modules
mod json_parse; // Fetch database data

// Main function
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.XethiumSoftware.Apptrium")
        .build();

    app.connect_activate(|app| {
        // Create a 1000x600 window
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

    //
    // Example: Get to know who is the author of vscode by uncommenting this!
    //
    // match json_parse::get_app_database_value("vscode", "Author") {
    //     Ok(value) => println!("{}", value),
    //     Err(e) => eprintln!("Error: {}", e),
    // }
    //
    app.run()
}
