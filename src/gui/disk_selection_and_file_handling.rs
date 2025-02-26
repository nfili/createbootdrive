fn setup_callbacks(gui_rc: Rc<RefCell<Gui>>) {
    // Signal 1: select_disk
    {
        let this = Rc::downgrade(&gui_rc);
        gui_rc.borrow_mut().select_disk.connect_map(move |disks| {
            if let Some(rc_gui) = this.upgrade() {
                let mut this = rc_gui.borrow_mut();
                this.validate_champ();
                if let Some(item) = disks.selected_item() {
                    if let Ok(string_object) = item.downcast::<gtk4::StringObject>() {
                        if string_object.string().to_string() != String::from("(AUCUN") {
                            this.set_disk_path(string_object.string().to_string().split(" ").collect::<Vec<&str>>()[0].to_string());
                        } else {
                            this.set_disk_path(String::new());
                        }
                    }
                }
            }
        });
    }

    // Signal 2: button_file clicked
    {
        let this = Rc::downgrade(&gui_rc);
        gui_rc.borrow_mut().button_file.connect_clicked(move |_| {
            if let Some(rc_gui) = this.upgrade() {
                let this_clone = rc_gui.clone();
                this_clone.borrow().create_file_chooser().run_async(move |chooser, response| {
                    if let Some(rc_gui) = this.upgrade() {
                        let mut this = rc_gui.borrow_mut();
                        chooser.close();
                        match response {
                            ResponseType::Ok => {
                                let file = chooser.file().unwrap();
                                let path = file.path().unwrap();
                                this.iso_size = calcule_size_image(&path);
                                this.set_path_iso(path.display().to_string());
                                this.label_iso.set_label(&format!(
                                    "{} ({})",
                                    path.file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap_or("N/A"),
                                    human_read(this.iso_size)
                                ));
                                this.validate_champ();
                            }
                            _ => {
                                this.label_iso.set_label(LABEL_ISO_NONE);
                                this.set_path_iso(String::new());
                                this.iso_size = 0;
                                this.btn_valider.set_sensitive(false);
                                this.is_data_valide = false;
                            }
                        }
                    }
                });
            }
        });
    }
}