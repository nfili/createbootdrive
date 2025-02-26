use glib::object_subclass;
use glib::prelude::ObjectExt;
use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::{ObjectImplExt, ObjectSubclass/*, ObjectSubclassExt*/};
use gtk4::subclass::prelude::WidgetImpl;
use gtk4::subclass::window::WindowImpl;
use gtk4::{Button, Window};
use std::cell::Cell;

#[derive(Default)]
pub struct DetailsView {
    pub btn_val: Button,
    pub status: Cell<bool>,
}
#[object_subclass]
impl ObjectSubclass for DetailsView {
    const NAME: &'static str = "DetailsView";
    type Type = super::DetailsView;
    type ParentType = Window;
}
impl ObjectImpl for DetailsView {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for DetailsView {}
impl WindowImpl for DetailsView {}