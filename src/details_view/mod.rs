mod imp;

use crate::utilitaires::create_button;
use glib::object::IsA;
use glib::prelude::ObjectExt;
use glib::property::{PropertyGet, PropertySet};
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk4::prelude::{BoxExt, ButtonExt, GridExt, GtkWindowExt, WidgetExt};
use gtk4::{Accessible, Actionable, Buildable, ConstraintTarget, Widget, Window};
use gtk4::{Align, Button, Label};
use std::path::Path;
use crate::APP_NAME;

glib::wrapper! {
        pub struct DetailsView(ObjectSubclass<imp::DetailsView>)
        @extends Window,Widget,
         @implements Accessible, Actionable, Buildable, ConstraintTarget;
    }
impl DetailsView {
    pub fn new(parent: &impl IsA<Window>) ->Self{
        glib::Object::builder::<DetailsView>()
            .property("title", APP_NAME)
            .property("transient-for", &parent)
            .property("default-width", 300)
            .property("default-height", 200)
            .property("hide-on-close", true)
            .build()
    }
    pub fn init(&mut self, path: String, iso: String, bs: String){
        self.set_status(false);
        let label_path = Label::new(Some("Périphérique :"));
        label_path.set_halign(Align::End);
        let label_iso = Label::new(Some("Image :"));
        label_iso.set_halign(Align::End);
        let label_bs = Label::new(Some("Taille du bloc :"));
        label_bs.set_halign(Align::End);
        let details_label = Label::new(Some(path.as_str()));
        details_label.set_halign(Align::End);
        let details_iso_label = Label::new(Some(Path::new(&iso).file_name().unwrap().to_str().unwrap()));
        details_iso_label.set_halign(Align::End);
        let details_bs_label = Label::new(Some(bs.as_str()));
        details_bs_label.set_halign(Align::End);
        let grid_top_level = gtk4::Grid::new();
        grid_top_level.set_column_homogeneous(false);
        grid_top_level.set_row_homogeneous(true);
        grid_top_level.set_column_spacing(10);
        grid_top_level.set_row_spacing(7);
        grid_top_level.set_margin_top(15);
        grid_top_level.set_margin_bottom(15);
        grid_top_level.set_margin_start(15);
        grid_top_level.set_margin_end(15);
        //grid_top_level.set_halign(Align::Center);
        grid_top_level.set_valign(Align::Center);
        grid_top_level.attach(&label_path,0,0,1,1);
        grid_top_level.attach(&label_iso,0,1,1,1);
        grid_top_level.attach(&label_bs,0,2,1,1);
        grid_top_level.attach(&details_label,1,0,1,1);
        grid_top_level.attach(&details_iso_label,1,1,1,1);
        grid_top_level.attach(&details_bs_label,1,2,1,1);

        let warning_label = Label::new(Some("Attention : Le media choisi va étre totalement effacé"));
        warning_label.set_wrap(false);
        warning_label.set_halign(Align::Center);
        warning_label.set_valign(Align::Center);
        warning_label.set_margin_start(5);
        warning_label.set_margin_end(5);

        let btn_close = create_button("Annuler","quit.svg","Fermer",true);
        let btn_valider = create_button("Valider","valider.svg","Valider",true);
        let box_btn_hor_interaction_top_level = gtk4::Box::new(gtk4::Orientation::Horizontal,5);
        box_btn_hor_interaction_top_level.set_halign(Align::Center);
        box_btn_hor_interaction_top_level.set_valign(Align::Center);
        box_btn_hor_interaction_top_level.set_margin_top(15);
        box_btn_hor_interaction_top_level.append(&btn_close);
        box_btn_hor_interaction_top_level.append(&btn_valider);

        let top_level = gtk4::Box::new(gtk4::Orientation::Vertical,5);
        top_level.set_halign(Align::Center);
        top_level.append(&grid_top_level);
        top_level.append(&warning_label);
        top_level.append(&box_btn_hor_interaction_top_level);

        self.set_child(Some(&top_level));

        //actions
        btn_close.connect_clicked({
            let mut this = self.clone();
            move |_| {
                this.close();
            }
        });
        btn_valider.connect_clicked({
            let mut this = self.clone();
            move |_| {
                this.set_status(true);
                this.hide();
                this.close();
            }
        });
    }
    fn create_btn(icon_name : &str, label_text: &str) -> Button{
        let btn_content = gtk4::Box::new(gtk4::Orientation::Horizontal,5);
        let btn_icon = gtk4::Image::from_icon_name(icon_name);
        let btn_label = Label::new(Some(label_text));
        btn_content.append(&btn_icon);
        btn_content.append(&btn_label);
        let btn = Button::new();
        btn.set_child(Some(&btn_content));
        btn
    }

    pub fn status(&self)  -> bool {
        self.imp().status.get()
    }
    fn set_status(&self, status: bool) {
        self.imp().status.set(status);
    }

    pub fn show(&self) {
        <DetailsView as AsRef<Window>>::as_ref(self).show();
    }
}