// src/progress_bar/mod.rs
mod imp;

use gtk4::{glib, subclass::prelude::*};
use gtk4::{prelude::*, Accessible, Actionable, Align, Box, Buildable, ConstraintTarget, Widget};

glib::wrapper! {
    pub struct MyProgressBar(ObjectSubclass<imp::MyProgressBar>)
    @extends Widget, Box,
    @implements Accessible, Buildable, ConstraintTarget, Actionable;
}

impl MyProgressBar {
    pub fn new() -> Self {
       glib::Object::builder().build()
    }
    fn init(&self){
        let imp = self.imp();
        //init progress_bar
        imp.progress_bar.set_show_text(true);
        imp.progress_bar.set_text(Some("En attente de validation..."));
        imp.progress_bar.set_width_request(300);

        imp.status.replace(false);
    }

    pub fn set_progress(&self, value: f64) {
        let imp = self.imp();
        imp.progress_bar.set_fraction(value);
    }
    pub fn set_progress_text(&self,text: &String){
        let imp = self.imp();
        imp.progress_bar.set_text(Some(text));
    }
    pub fn set_status(&mut self, status: bool) {
        let mut imp = self.imp();
        imp.status.replace(status);
    }
    pub fn status(&self) -> bool {
        let imp = self.imp();
        *imp.status.borrow()
    }
}