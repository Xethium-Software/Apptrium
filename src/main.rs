/// theridev was here
/// Nov 18: Main
/// Nov 19: Apptrium-Legacy-ToRust

// barkotbb was here
/// Nov 19: Added Dark Mode toggle functionality
/// - Implemented a button to switch between light and dark themes.
/// - Added CSS providers for light and dark themes.
/// - Fixed layout margins for better UI appearance.

// I hate this warning
#[allow(unused_imports)]

// Imports
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider};
use gtk::gdk;
use log::{debug, error, info, warn};

// Imports: Apptrium modules
mod json_parse; // Fetch database data

static mut IS_DARK_MODE: bool = false;

// Main function
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.Xethium.Apptrium")
        .build();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    app.connect_activate(|app| {
        // Create a 1000x600 window
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(1000)
            .default_height(600)
            .resizable(false)
            .title("Apptrium")
            .build();

        // Create the DarkModeButton
        let dark_mode_button = Button::new();
        dark_mode_button.set_label("Toggle Dark Mode");

        // Set margins for button
        dark_mode_button.set_margin_top(180);
        dark_mode_button.set_margin_bottom(180);
        dark_mode_button.set_margin_end(180);
        dark_mode_button.set_margin_start(180);

        // Connect the button click to toggle dark mode
        dark_mode_button.connect_clicked(|_| {
            toggle_dark_mode();
        });

        // Apply initial CSS
        light_bg_css();

        // Set the button as the window child
        window.set_child(Some(&dark_mode_button));

        // Show the window
        window.present();
    });

    app.run()
}

fn dark_bg_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Load the CSS from src/darkmode.css
    provider.load_from_data(include_str!("./Style/darkmode.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}

fn light_bg_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Load the CSS from src/lightmode.css
    provider.load_from_data(include_str!("./Style/lightmode.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}

fn toggle_dark_mode() {
    unsafe {
        if !IS_DARK_MODE {
            debug!("Theme switched to Light Mode.");
            dark_bg_css();
            IS_DARK_MODE = true;
        } else {
            debug!("Theme switched to Dark Mode");
            light_bg_css();
            IS_DARK_MODE = false;
        }
    }
}
