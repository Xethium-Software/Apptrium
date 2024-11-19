/// theridev was here
/// Nov 18: Main
/// Nov 19: Apptrium-Legacy-ToRust

// barkotbb was here
/// Nov 19: Added Dark Mode toggle functionality
/// - Implemented a button to switch between light and dark themes.
/// - Added CSS providers for light and dark themes.
/// - Fixed layout margins for better UI appearance.

// Imports
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider};
use gtk::gdk;

// Imports: Apptrium modules
mod json_parse; // Fetch database data

static mut IS_DARK_MODE: bool = false;

// Main function
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.Xethium.Apptrium")
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
            settings_popup();
            toggle_dark_mode();
        });

        // Apply initial CSS
        LightBgCss();

        // Set the button as the window child
        window.set_child(Some(&dark_mode_button));

        // Show the window
        window.present();
    });

    app.run()
}

fn DarkBgCss() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Load the CSS from src/darkmode.css
    provider.load_from_data(include_str!("./Style/darkmode.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}

fn LightBgCss() {
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
            println!("Switching to Light Mode!");
            DarkBgCss();
            IS_DARK_MODE = true;
        } else {
            println!("Switching to Dark Mode!");
            LightBgCss();
            IS_DARK_MODE = false;
        }
    }
}

fn settings_popup() {
    // Create the setting pop-up dialog.
    let setting_window = gtk::Dialog::new();
    setting_window.set_title(Some("Settings"));
    setting_window.set_default_size(800, 600);
    setting_window.set_resizable(false);

    let content_area = setting_window.content_area();
    let label = gtk::Label::new(Some("This is setting."));
    content_area.append(&label);

    setting_window.show();
}