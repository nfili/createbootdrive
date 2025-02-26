use std::cell::RefCell;
use std::rc::Rc;
// src/custom_widget/imp.rs
use gtk4::{glib, prelude::*, subclass::prelude::*, Box, Label, ProgressBar, Orientation};


pub struct MyProgressBar {
    pub progress_bar: ProgressBar,
    pub status: Rc<RefCell<bool>>,
}

#[glib::object_subclass]
impl ObjectSubclass for MyProgressBar {
    const NAME: &'static str = "MyProgressBar";
    type Type = super::MyProgressBar;
    type ParentType = Box;
}

impl ObjectImpl for MyProgressBar {
    fn constructed(&self) {
        self.parent_constructed();
        let container = Box::new(Orientation::Vertical, 0);
        container.append(&self.progress_bar);
        self.obj().append(&container);
        // Initialize the progress bar and label settings
        self.obj().init();
    }
}
impl Default for MyProgressBar {
    fn default() -> Self {
        Self {
            progress_bar: ProgressBar::new(),
            status: Rc::new(RefCell::new(false)),
        }
    }
}
impl WidgetImpl for MyProgressBar {}
impl BoxImpl for MyProgressBar {}
unsafe impl Send for MyProgressBar {}