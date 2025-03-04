use gtk::prelude::*;
use gtk::{ApplicationWindow, Image};
use std::error::Error;

pub fn gui_main(input_file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let application = gtk::Application::new(Some("com.github.marinm.songrec"),
        gio::ApplicationFlags::HANDLES_OPEN)
        .expect("Application::new failed");

    application.connect_startup(move |application| {
        let main_window: ApplicationWindow = ApplicationWindow::new(application);
        main_window.set_title("Album Art Display");
        main_window.set_default_size(800, 600);

        // Create an image widget to hold album art
        let album_art = Image::from_file("path/to/your/album_art.jpg");

        // Add the album art directly to the main window
        main_window.add(&album_art);

        // Set the window to fullscreen
        main_window.fullscreen();

        // Show all widgets
        main_window.show_all();
    });

    application.connect_activate(move |application| {
        let main_window = &application.get_windows()[0];
        main_window.present();
    });

    if let Some(input_file_string) = input_file {
        application.run(&["songrec".to_string(), input_file_string.to_string()]);
    } else {
        application.run(&[]);
    }

    Ok(())
}
