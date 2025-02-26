use create_boot_drive::new_app::new_builder;
mod resources;

fn main() -> glib::ExitCode {
    resources::register_resources();
    new_builder()
}