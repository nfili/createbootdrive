use crate::gui::*;
use crate::APP_ID;

use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::prelude::NativeDialogExt;
// Pré-requis pour les objets et événements GTK
use gtk4::Application;

pub fn new_builder() -> glib::ExitCode {
    // Créez une instance de l'application GTK
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run()
}
fn build_ui(app: &Application) {
    let gui= Gui::new(app);
    gui.show();
}

