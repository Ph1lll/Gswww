use adw::gtk::{Box, Button, FileChooserAction, FileChooserNative, Orientation};
use adw::{glib, prelude::*, Application, ApplicationWindow, Window};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.github.dwogo.Gswww")
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
        .orientation(Orientation::Vertical)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gswww")
        .content(&main_box)
        .build();

    // Create a button with label and margins
    let dialog_button = Button::builder()
        .label("Select Folder")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .halign(adw::gtk::Align::End)
        .valign(adw::gtk::Align::Center)
        .build();

    let dialog = FileChooserNative::new(
        Some("Select Folder"),
        Window::NONE,
        FileChooserAction::SelectFolder,
        Some("Select"),
        Some("Cancel"),
    );

    dialog.connect_response(move |dialog, response| {
        if response == adw::gtk::ResponseType::Accept {
            if let Some(file) = dialog.file() {
                if let Some(path) = file.path() {
                    println!("Selected file: {:?}", path);
                }
            }
        }
        dialog.hide();
    });

    dialog_button.connect_clicked(move |_| {
        dialog.show();
    });

    let image_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Append the 2 constantly seen items
    main_box.append(&dialog_button);
    main_box.append(&image_box);

    window.present();
}
