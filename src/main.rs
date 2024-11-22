#![allow(unused_imports)]

/// theridev was here
/// Nov 18: master
/// Nov 19: Apptrium-Legacy-ToRust
/// Nov 19: master
/// Nov 20: master
/// - Added preferences.json fetching for dark / light mode.

// ShaderHex was here
/// Nov 19: DarkMode
/// Added Dark Mode toggle functionality
/// - Implemented a button to switch between light and dark themes.
/// - Added CSS providers for light and dark themes.
/// - Fixed layout margins for better UI appearance.
/// Nov 22
/// - Added Setting pop-up.
/// - Added Dark / Light function.
/// - I organized the code into different files.


use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, CssProvider, Fixed};
use log::{debug, error};

mod json_parse; // Fetch database data
mod setting;

static mut IS_DARK_MODE: bool = false;

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

        let layout = Fixed::new();

        // Create the Settings button
        let settings_button = Button::new();
        settings_button.set_label("Settings");
        settings_button.set_widget_name("settings_button");

        // Connect the button click to open settings popup
        settings_button.connect_clicked(|_| {
            setting::settings_popup();
        });

        // Fetch initial preferences and apply theme
        match json_parse::get_preferences_value("Theme", "darkMode") {
            Ok(value) => {
                let theme = match value.to_lowercase().as_str() {
                    "true" => Some(false),
                    "false" => Some(true),
                    _ => {
                        error!("Invalid Theme:DarkMode value: {}. Defaulting to Light Mode.", value);
                        Some(true) // Fall back to Light Mode.
                    }
                };

                unsafe {
                    IS_DARK_MODE = theme.unwrap_or(false);
                }

                // Apply the determined theme
                setting::toggle_dark_mode();
            }
            Err(e) => eprintln!("Error fetching preferences: {}", e),
        }

        // Load CSS styles
        setting::loading_style_css();

        layout.put(&settings_button, 870.0, 50.0);
        window.set_child(Some(&layout));

        // Show the window
        window.present();
        debug!("Main: Window created!");
    });

    app.run()
}

