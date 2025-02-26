use glib::Bytes;

pub fn register_resources() {
    let resource_data: &[u8] = include_bytes!("resources/resources.gresource");
    let resource = gio::Resource::from_data(&Bytes::from(&resource_data[..]))
        .expect("Failed to load resource");
    gio::resources_register(&resource);
}