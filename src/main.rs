use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use directories::UserDirs;
use gtk::{glib, Box, Button, Image};
use std::rc::Rc;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("dev.dwogo.Gswww")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let main_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    // Create a button with label and margins
    let button = Button::builder()
        .label("Open File Dialog")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let image_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    main_box.append(&button);
    main_box.append(&image_box);

    button.connect_clicked(glib::clone!(@weak image_box => move|_| button_pushed(&image_box)));

    let window = Rc::new(
        ApplicationWindow::builder()
            .application(app)
            .title("Gswww")
            .content(&main_box)
            .build(),
    );

    window.present();
}

fn button_pushed(image_box: &gtk::Box) {
    if let Some(picture_directory) = UserDirs::new() {
        let mut directory = std::path::PathBuf::new();
        directory.push({
            let user_dir = format!("{:?}", picture_directory.picture_dir().unwrap());
            format!("{}/Wallpapers", &user_dir[1..user_dir.len() - 1])
        });
        println! {"{}", directory.display()};

        let paths = std::fs::read_dir(directory).unwrap();

        for path in paths {
            let image0 = Image::from_file(path.unwrap().path());
            image_box.append(&image0);
        }
    }
}
