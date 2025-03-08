use gdk::EventButton;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::ResponseType;
use gettextrs::gettext;
use gdk_pixbuf::Pixbuf;
use std::error::Error;
use std::sync::mpsc;
use std::cell::RefCell;
use std::rc::Rc;
use chrono::Local;
use std::sync::RwLock;
use std::sync::Arc;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
#[cfg(feature = "mpris")]
use mpris_player::PlaybackStatus;

use crate::core::microphone_thread::microphone_thread;
use crate::core::processing_thread::processing_thread;
use crate::core::http_thread::http_thread;
use crate::core::thread_messages::{*, GUIMessage::*};

use crate::gui::song_history_interface::FavoritesInterface;
use crate::gui::song_history_interface::{SongRecordInterface, RecognitionHistoryInterface};
use crate::utils::filesystem_operations::obtain_favorites_csv_path;
#[cfg(feature = "mpris")]
use crate::utils::mpris_player::{get_player, update_song};

use crate::gui::preferences::{PreferencesInterface, Preferences};
use crate::utils::csv_song_history::SongHistoryRecord;
use crate::utils::filesystem_operations::obtain_recognition_history_csv_path;

#[cfg(windows)]
use std::os::windows::process::CommandExt;

pub fn gui_main(recording: bool, input_file: Option<&str>, enable_mpris_cli: bool) -> Result<(), Box<dyn Error>> {
    let application = gtk::Application::new(Some("com.github.marinm.songrec"),
        gio::ApplicationFlags::HANDLES_OPEN)
        .expect(&gettext("Application::new failed"));

    application.connect_startup(move |application| {
        // Create the main application window
        let main_window: gtk::ApplicationWindow = gtk::ApplicationWindow::new(application);
        main_window.set_title("Album Art Full Screen");
        main_window.set_default_size(800, 600);

        // Create an Image widget to display the album art
        let album_art_image = gtk::Image::new();

        // Load the album art (replace with your album art path)
        let album_art_path = "path/to/your/album/art.jpg"; // Update with the actual path
        let pixbuf = Pixbuf::from_file(album_art_path).expect("Failed to load album art");
        album_art_image.set_from_pixbuf(Some(&pixbuf));

        // Add the image to the main window
        main_window.add(&album_art_image);
        
        // Set the window to full screen
        main_window.fullscreen();

        // Show all widgets in the window
        main_window.show_all();
    });

    application.connect_activate(move |application| {
        let main_window = &application.get_windows()[0];

        // Raise the existing window to the top whenever a second
        // GUI instance is attempted to be launched
        main_window.present();

        // Close all windows when the main window is closed
        main_window.connect_delete_event(|_, _| {
            for window in gtk::Window::list_toplevels() {
                if let Ok(window) = window.downcast::<gtk::Window>() {
                    window.close();
                }
            }
            gtk::Inhibit(false) // Do not inhibit the default delete event behavior
        });
    });

    if let Some(input_file_string) = input_file {
        application.run(&[input_file_string]);
    } else {
        application.run(&[]);
    }

    Ok(())
}
