use glib::subclass::object::ObjectImpl;
use glib::subclass::prelude::{ObjectImplExt, ObjectSubclass, ObjectSubclassType};
use glib::object_subclass;
use gtk4::subclass::widget::WidgetImpl;
use gtk4::subclass::window::WindowImpl;
use gtk4::Window;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct FinishView {
    pub status: Rc<RefCell<bool>>,
}
#[object_subclass]
impl ObjectSubclass for FinishView {
    const NAME: &'static str = "FinishView";
    type Type = super::FinishView;
    type ParentType = Window;
}

impl ObjectImpl for FinishView {
    fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for FinishView {}
impl WindowImpl for FinishView {}