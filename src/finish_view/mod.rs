mod imp;

use std::path::Path;
use gio::prelude::ApplicationExt;
use glib::object::IsA;
use glib::prelude::Cast;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk4::prelude::{BoxExt, ButtonExt, GtkWindowExt, WidgetExt};
use gtk4::{Accessible, Actionable, Box, Buildable, Button, ConstraintTarget, Image, Label, Widget, Window};
use crate::utilitaires::get_resource_path;

glib::wrapper! {
    pub struct FinishView(ObjectSubclass<imp::FinishView>)
        @extends Window,Widget,
        @implements Accessible, Actionable, Buildable, ConstraintTarget;
    }
impl FinishView{
    pub(crate) fn new(parent: &impl IsA<Window>) -> Self {
        glib::Object::builder::<FinishView>()
            .property("title", "Périphérique créé")
            .property("transient-for", &parent)
            .property("default-width", 350)
            .property("default-height", 200)
            .property("hide-on-close", true)
            .build()
    }
    pub fn init(&self, disk: String, iso: String){
        let img = Image::from_resource(get_resource_path("BootDiskGraphic.svg").as_str());
        img.set_halign(gtk4::Align::Start);
        img.set_valign(gtk4::Align::Center);
        img.set_size_request(128,128);

        let label = Label::new(Some(&format!("l'image {} a correctement été copiée sur le périphérique {}",
                                             Path::new(&iso).file_name().unwrap().to_str().unwrap(),disk)));
        label.set_wrap(true);
        label.set_hexpand(true);
        label.set_vexpand(true);
        label.set_halign(gtk4::Align::Start);
        label.set_valign(gtk4::Align::Center);

        let box_finish = Box::new(gtk4::Orientation::Horizontal,5);
        box_finish.set_hexpand(true);
        box_finish.set_vexpand(true);
        box_finish.append(&img);
        box_finish.append(&label);

        let btn_close_box = Box::new(gtk4::Orientation::Horizontal,5);
        let btn_close = Button::new();
        let btn_close_label = Label::new(Some("Quitter"));
        let btn_close_ico = Image::from_resource(get_resource_path("quit.svg").as_str());
        btn_close.set_hexpand(true);
        /*btn_close_box.set_vexpand(true);*/
        btn_close.set_halign(gtk4::Align::Center);
        btn_close.set_margin_bottom(10);
        btn_close_box.append(&btn_close_ico);
        btn_close_box.append(&btn_close_label);
        btn_close.set_child(Some(&btn_close_box));

        let top_level = Box::new(gtk4::Orientation::Vertical,5);
        top_level.append(&box_finish);
        top_level.append(&btn_close);

        self.set_child(Some(&top_level));
        let this=self.clone();
        btn_close.connect_clicked(move |_| {
            let this = this.clone();
            this.set_status(true);
            this.close();
        });
    }
    pub fn show(&self) { self.upcast_ref::<Window>().show();}
    pub fn close(&self) {self.upcast_ref::<Window>().close();}
    pub fn status(&self) -> bool {self.imp().status.borrow().clone()}
    fn set_status(&self, status: bool) {self.imp().status.replace(status);}
}