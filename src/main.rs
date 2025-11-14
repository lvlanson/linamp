mod settings;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, ListBox, PolicyType, ScrolledWindow};
use settings::library::MusicLibrary;
use std::path::{Path, PathBuf};

const APP_ID: &str = "org.lvlanson.linamp";

fn main() {
    // application window
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    // setup path for examplary music library, dummy string
    let path: PathBuf = Path::new("/home/thomas/Music").to_path_buf();

    let mut music_lib = MusicLibrary::new(path);
    music_lib.scan_to_library();

    let list_box = ListBox::new();

    for path in music_lib.get_files() {
        let label = Label::new(Some(path));
        list_box.append(&label);
    }

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_width(360)
        .child(&list_box)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Linamp")
        .default_width(600)
        .default_height(300)
        .child(&scrolled_window)
        .build();

    window.present();
}
