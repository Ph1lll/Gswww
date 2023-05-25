use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("dev.dwogo.Gswww")
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gswww")
        .build();

    // Create a button with label and margins
    let button = Button::builder()
        .label("Open File Dialog")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    button.connect_clicked(|_| gtk::FileDialog);
    window.set_child(Some(&button));

    window.present();
}
