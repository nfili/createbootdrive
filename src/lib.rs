
pub mod new_app;
pub mod gui;
mod details_view;
mod disks;
mod utilitaires;
mod dd;
mod progress_bar;
mod finish_view;
mod resources;

pub const APP_ID: &str = "org.gtk_rs.createbootdrive";
pub const APP_NAME: &str = "Create Boot Drive";
pub const APP_SIZES: (i32, i32) = (420,400);
