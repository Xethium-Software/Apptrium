use crate::IS_DARK_MODE; // Import IS_DARK_MODE
use gtk::prelude::*;
use gtk::{CssProvider, Dialog, Label, Fixed, Button};
use gtk::gdk;
use log::{debug, error};


pub fn settings_popup() {
    // Create the settings popup dialog
    let setting_window = Dialog::builder()
        .title("Settings")
        .default_width(800)
        .default_height(600)
        .resizable(false)
        .build();

    // display "Setting" on the top middle
    let content_area = setting_window.content_area();
    let title_label = Label::new(Some("Settings"));
    content_area.append(&title_label);

    // display "Theme" text
    let layout = Fixed::new();
    let theme_label = Label::new(Some("Theme (Experimental)"));
    layout.put(&theme_label, 0.0, 50.0);
    content_area.append(&layout);

    // Draw the line
    let line_widget = gtk::Label::new(None);
    line_widget.set_widget_name("line");
    content_area.append(&line_widget);

    // Display the text "Dark / Light theme:"
    let theme_layout = Fixed::new();
    let theme_label = Label::new(Some("Dark / Light theme: "));
    theme_layout.put(&theme_label, 0.0, 10.0);
    content_area.append(&theme_layout);
    
    // Change Theme button
    let dark_light_button = Button::new();
    dark_light_button.set_label("Change Theme");
    content_area.append(&dark_light_button);
    dark_light_button.set_widget_name("dark_light_button");

    // Call toggle_dark_mode() function when clicked on the button
    dark_light_button.connect_clicked(|_| {
        toggle_dark_mode();
    });

    setting_window.show();
}

pub fn dark_bg_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Load the CSS from src/darkmode.css
    provider.load_from_data(include_str!("./Style/darkmode.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}

pub fn light_bg_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Load the CSS from src/lightmode.css
    provider.load_from_data(include_str!("./Style/lightmode.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}

pub fn toggle_dark_mode() {
    unsafe {
        if IS_DARK_MODE {
            debug!("Theme switched to Light Mode.");
            light_bg_css();
            IS_DARK_MODE = false;
        } else {
            debug!("Theme switched to Dark Mode.");
            dark_bg_css();
            IS_DARK_MODE = true;
        }
    }
}

pub fn loading_style_css() {
    let display = gdk::Display::default().expect("Could not get default display.");
    let provider = CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    // Load the CSS from src/style.css
    provider.load_from_data(include_str!("./Style/style.css"));
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}
