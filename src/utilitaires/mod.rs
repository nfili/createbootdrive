use std::path::Path;
use byte_unit::UnitType;
use gtk4::prelude::{BoxExt, ButtonExt, WidgetExt};

pub fn human_read(s: u64) -> String {
    let byte = byte_unit::Byte::from_u64(s);
    let adjust_byte = byte.get_appropriate_unit(UnitType::Binary);
    format!("{adjust_byte:.2}")
}
pub fn check_size_image(size_image: u64, size_disk: u64) -> bool {
    size_image < size_disk
}
pub fn calcule_size_image(path: &Path) -> u64 {
    println!("{:?}", path);
    path.metadata().unwrap().len()
}

pub fn get_resource_path(resource: &str) -> String {
    let base = "/org/gtk_rs/createbootdrive/".to_owned() +resource;
    String::from(base)
}

pub fn create_button(label: &str, icon: &str,tooltip_text: &str, label_state:bool) -> gtk4::Button {
    let button = gtk4::Button::new();
    let icon = gtk4::Image::from_resource(get_resource_path(icon).as_str());
    icon.set_halign(gtk4::Align::Fill);
    icon.set_valign(gtk4::Align::Fill);
    // icon.set_size_request(28, 28);
    button.set_tooltip_text(Some(tooltip_text));

    let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    button_box.append(&icon);

    if label_state {
        let label = gtk4::Label::new(Some(label));
        button_box.append(&label);
    }
    button.set_child(Some(&button_box));

    button
}