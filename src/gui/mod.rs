use crate::dd::run_dd_command;
use crate::details_view::DetailsView;
use crate::disks::Disks;
use crate::finish_view::FinishView;
use crate::progress_bar::MyProgressBar;
use crate::utilitaires::{calcule_size_image, check_size_image, create_button, human_read};
use crate::APP_NAME;
use gio::prelude::{FileExt, SettingsExt};
use glib::object::Cast;
use glib::prelude::StaticType;
use gtk4::prelude::{BoxExt, ButtonExt, DialogExtManual, EditableExt, FileChooserExt, GridExt, GtkWindowExt, WidgetExt};
use gtk4::Entry;
use gtk4::{Align, Application, ApplicationWindow, Button, DropDown, FileChooserAction, Grid, Label, ResponseType, StringList};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::string::String;
use std::sync::{Arc, Mutex};

const LABEL_ISO_NONE: &str = "pas d'images choisie";

enum GuiField {
    IsoPath(String),
    DiskPath(String),
    BsValue(String),
}
#[derive(Clone)]
pub struct Gui{
    pub window:ApplicationWindow,
    select_disk: DropDown,
    select_bs: DropDown,
    label_iso: Label,
    is_data_valide: bool,
    disks:Disks,
    disk_path: Rc<RefCell<String>>,
    iso_path: Rc<RefCell<String>>,
    iso_size: u64,
    btn_valider: Button,
    size_bs: Entry,
    bs_value: Rc<RefCell<String>>,
    progress_bar: MyProgressBar,
}

impl Gui{
    fn default(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(APP_NAME)
            /*.default_width(APP_SIZES.1)
            .default_height(APP_SIZES.0)
            */.build();

        let label_iso=Label::new(Some(LABEL_ISO_NONE));

        let mut disks = Disks::new();
        disks.set_disks();
        let select_disk = Self::create_drop_down(disks.disks());

        // liste des unités K pour kilo, M pour Mega
        let bs=vec![String::from("K"), String::from("M")];
        let select_bs = Self::create_drop_down(bs);

        let btn_valider= Button::new();

        let size_bs = Self::create_numeric_entry();

        let progress_bar = MyProgressBar::new();

        Self{
            window,
            select_disk,
            select_bs,
            label_iso,
            is_data_valide: false,
            disks,
            disk_path: Rc::new(RefCell::new(String::new())),
            iso_path: Rc::new(RefCell::new(String::new())),
            iso_size: 0,
            btn_valider,
            size_bs,
            bs_value: Rc::new(RefCell::new(String::new())),
            progress_bar,
        }
    }
    pub fn new(app: &Application) ->Self{
        let mut gui = Self::default(app);

        let label_intro = Label::new(Some("Création d'un support bootable linux/freeBSD/etc..., sur un périphérique USB, carte SD, etc..."));
        label_intro.set_wrap(true);
        label_intro.set_max_width_chars(60);
        label_intro.set_halign(Align::Center);
        label_intro.set_hexpand(true);
        label_intro.set_margin_top(20);
        label_intro.set_margin_bottom(15);


        // Création du module de selection du périphérique
        let label = Label::new(Some("Choix du périphérique : "));
        label.set_halign(Align::Start);
        label.set_hexpand(true);
        label.set_margin_start(15);
        let button_disk = create_button("","refresh.svg","Actualiser les périphériques",false);
        let device_select_first_level = gtk4::Box::new(gtk4::Orientation::Horizontal,5);
        device_select_first_level.set_hexpand(true);
        device_select_first_level.set_halign(Align::End);
        device_select_first_level.set_margin_end(15);
        device_select_first_level.set_margin_bottom(15);
        device_select_first_level.append(&gui.select_disk);
        device_select_first_level.append(&button_disk);

        // Création du module de définition de la taille du bloc
        let label_bs = Label::new(Some("Taille du bloc (en octets) : "));
        label_bs.set_halign(Align::Start);
        label_bs.set_hexpand(true);
        label_bs.set_margin_start(15);
        let bs_def_first_level = gtk4::Box::new(gtk4::Orientation::Horizontal,5);
        bs_def_first_level.set_hexpand(true);
        bs_def_first_level.set_halign(Align::End);
        bs_def_first_level.set_margin_end(15);
        bs_def_first_level.set_margin_bottom(15);
        bs_def_first_level.append(&gui.size_bs);
        bs_def_first_level.append(&gui.select_bs);
        
        // Creation du module de sélection de l'image
        let iso = gtk4::Label::builder().label("Sélectionner l'image à utiliser : ").build();
        iso.set_halign(Align::Start);
        iso.set_hexpand(true);
        iso.set_margin_start(15);
        let button_file = create_button("","iso_add.svg","Sélectionner l'image",false);
        let iso_select_first_level= gtk4::Box::new(gtk4::Orientation::Horizontal,5);
        iso_select_first_level.set_hexpand(true);
        iso_select_first_level.set_halign(Align::End);
        iso_select_first_level.set_margin_end(15);
        iso_select_first_level.set_margin_bottom(15);
        iso_select_first_level.append(&gui.label_iso);
        iso_select_first_level.append(&button_file);

        //
        let label_progress = Label::new(Some("Progression : "));
        label_progress.set_halign(Align::Start);
        label_progress.set_hexpand(true);
        label_progress.set_margin_start(15);
        gui.progress_bar.set_halign(Align::Center);
        gui.progress_bar.set_hexpand(true);
        gui.progress_bar.set_margin_bottom(15);

        let btn_valider = Gui::create_btn_validate(&gui);
        btn_valider.set_halign(Align::Center);
        btn_valider.set_hexpand(true);
        btn_valider.set_margin_bottom(15);

        gui.btn_valider=btn_valider;




        let grid = Grid::new();
        grid.set_column_spacing(5);
        grid.set_row_spacing(5);
        grid.attach(&label_intro, 0, 0, 2, 1);
        grid.attach(&label, 0, 1, 2, 1);
        grid.attach(&device_select_first_level, 0, 2, 2, 1);
        grid.attach(&label_bs, 0, 3, 2, 1);
        grid.attach(&bs_def_first_level, 0, 4, 2, 1);
        grid.attach(&iso, 0, 5, 2, 1);
        grid.attach(&iso_select_first_level, 0, 6, 2, 1);
        grid.attach(&label_progress, 0, 7, 2, 1);
        grid.attach(&gui.progress_bar, 0, 8, 2, 1);
        grid.attach(&gui.btn_valider, 0, 9, 2, 1);

        gui.window.set_child(Some(&grid));

        //CONNECTION DES SIGNAUX
        gui.select_disk.connect_map({
            let mut this = gui.clone();
            move |disks| {
                let mut this = this.clone();
                this.validate_champ();
                if let Some(item) = disks.selected_item() {
                    if let Ok(string_object) = item.downcast::<gtk4::StringObject>() {
                        if string_object.string().to_string() != String::from("(AUCUN") {
                            this.set_field(GuiField::DiskPath(string_object.string().to_string().split(" ").collect::<Vec<&str>>()[0].to_string()));
                        }
                        else {
                            this.set_field(GuiField::DiskPath(String::new()));
                        }
                    }
                }
            }
        });

        gui.select_disk.connect_selected_item_notify({
            let mut this = gui.clone();
            move |disks| {
                let mut this = this.clone();
                this.validate_champ();
                if let Some(item) = disks.selected_item() {
                    if let Ok(string_object) = item.downcast::<gtk4::StringObject>() {
                        if string_object.string().to_string() != String::from("(AUCUN") {
                            this.set_field(GuiField::DiskPath(string_object.string().to_string().split(" ").collect::<Vec<&str>>()[0].to_string()));
                        }
                        else {
                            this.set_field(GuiField::DiskPath(String::new()));
                        }
                    }
                }
            }
        });

        gui.select_bs.connect_map({
            let mut this = gui.clone();
            move |bs| {
                let mut this = this.clone();
                this.validate_champ();
                if let Some(item) = bs.selected_item() {
                    if let Ok(string_object) = item.downcast::<gtk4::StringObject>() {
                        if string_object.string().to_string() != String::from("(AUCUN") {
                            this.set_field(GuiField::BsValue(string_object.string().to_string()));
                        }
                    }
                }
            }
        });
        gui.select_bs.connect_selected_notify({
            let mut this = gui.clone();
            move |bs| {
                let mut this = this.clone();
                this.validate_champ();
                if let Some(item) = bs.selected_item() {
                    if let Ok(string_object) = item.downcast::<gtk4::StringObject>() {
                        if string_object.string().to_string() != String::from("(AUCUN") {}
                        this.set_field(GuiField::BsValue(string_object.string().to_string()));
                    }
                }
            }
        });
        gui.size_bs.connect_changed({
            let mut this = gui.clone();
            move |bs| {
                let mut this = this.clone();
                this.validate_champ();
            }
        });
        button_disk.connect_clicked({
            let mut this = gui.clone();
            move |_| {
                let mut this = this.clone();
                this.disks.set_disks();
                this.select_disk.set_model(Some(&Self::create_list_drop_down(this.disks.disks())));
            }
        });
        button_file.connect_clicked({
            let mut this = gui.clone();
            move |_| {
                let mut this = this.clone();
                this.create_file_chooser().run_async(move |obj, answer| {
                    obj.close();
                    match answer {
                        ResponseType::Ok => {
                            this.iso_size=calcule_size_image(Path::new(obj.file().unwrap().path().unwrap().to_str().unwrap()));
                            this.set_field(GuiField::IsoPath(obj.file().unwrap().path().unwrap().to_str().unwrap().to_string()));
                            this.label_iso.set_label(
                                format!("{} ({})",obj.file().unwrap().path().unwrap().file_name().unwrap().to_str().unwrap(),
                                        human_read(this.iso_size)).as_str());
                            this.validate_champ();
                        }
                        _ => {
                            this.label_iso.set_label(LABEL_ISO_NONE);
                            this.set_field(GuiField::IsoPath(String::new()));
                            this.iso_size = 0;
                            this.btn_valider.set_sensitive(false);
                            this.is_data_valide = false;
                        }
                    }
                });
            }
        }
        );

        //progress_bar.connect_map(move|obj| dd::run_dd_command(Arc::new(Mutex::new(obj.clone()))));
        gui
}

fn create_drop_down(model:Vec<impl ToString>) -> DropDown{
        let drop_down = DropDown::builder().build();
        let exp = gtk4::PropertyExpression::new(
            gtk4::StringObject::static_type(),
            None::<gtk4::Expression>,
            "string",
        );
        drop_down.set_model(Some(&Self::create_list_drop_down(model)));
        drop_down.set_expression(Some(&exp));
        drop_down.set_enable_search(false);
        drop_down.set_show_arrow(false);

        drop_down
    }
    // fonction privée créant la fenetre de selection d'image
    fn create_file_chooser(&self) -> gtk4::FileChooserDialog{
        let filter = gtk4::FileFilter::new();
        filter.add_mime_type("iso");
        filter.add_pattern("*.img");
        let chooser =gtk4::FileChooserDialog::new(Some("Sélectionner l'image"),
                                     Some(&self.window),
                                     FileChooserAction::Open,
                                     &[("ok",ResponseType::Ok),("annuler",ResponseType::Cancel)]);
        chooser.add_filter(&filter);
        chooser.set_modal(true);
        chooser.set_transient_for(Some(&self.window));
        chooser.set_destroy_with_parent(true);
        chooser
    }
    fn create_list_drop_down(list: Vec<impl ToString>) -> StringList{
        let list_tmp = list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        StringList::new(list_tmp.iter().map(|s| s as &str).collect::<Vec<_>>().as_slice())
    }
    fn create_numeric_entry() -> Entry {
        let entry = Entry::new();

        entry.connect_changed(|entry| {
            let text = entry.text();
            let filtered_text: String = text.chars().filter(|c| c.is_numeric()).collect();

            if text != filtered_text {
                entry.set_text(&filtered_text);
                entry.set_position(filtered_text.len() as i32);
            }
        });
        entry
    }
    pub fn create_btn_validate(&self) -> Button {
        let btn = Button::new();
        btn.set_label("Créer le disque bootable");

        btn.connect_clicked({
            let mut this = self.clone();
            move |_| {
                let mut this = this.clone();
                if this.validate_champ(){
                    println!("disk path : {}",this.disk_path());
                    println!("iso path : {}",this.iso_path());
                    println!("bs value : {}",this.bs_value());
                    println!("size bs : {}",this.size_bs.text());
                    let bs= format!("{}{}",this.size_bs.text(),this.bs_value());
                    let mut details = DetailsView::new(&this.window);
                    details.init(this.disk_path(),this.iso_path(),bs);
                    details.show();

                    details.connect_unmap(
                        {
                            let mut this = this.clone();
                            move |obj| {
                                obj.close();
                                let mut this = this.clone();
                                if obj.status() {
                                    let finish_view = FinishView::new(&this.window);
                                    finish_view.init(this.disk_path(),this.iso_path());

                                    run_dd_command(Arc::new(Mutex::new(this.progress_bar.clone())),
                                                   this.iso_path().as_ref(),
                                                   this.disk_path().as_ref(),
                                                   this.bs_value.borrow().as_ref(),
                                                   this.iso_size,
                                                   Arc::new(Mutex::new(finish_view.clone())),
                                    );
                                    //this.window.close();
                                    finish_view.connect_unmap({
                                        let mut this = this.clone();
                                        move |obj| {
                                            let mut this = this.clone();
                                            this.window.close();
                                        }
                                    });
                                }
                            }
                        }
                    );
                }
            }
        });
        btn
    }
    fn validate_champ(&mut self) -> bool {
        if let Some(item) =  self.select_disk.selected_item(){
            if let Ok(string_object) = item.downcast::<gtk4::StringObject>() {
                if string_object.string().to_string() != String::from("(AUCUN") {
                    let disk_selected = self.disks.get_disk_by_device(
                        string_object.string().split(" ").collect::<Vec<&str>>()[0]);
                    if check_size_image(self.iso_size, disk_selected.unwrap_or(&Default::default()).size) {
                        if self.select_disk.selected_item().is_some() &&
                            self.select_bs.selected_item().is_some() &&
                            !self.size_bs.text().is_empty() &&
                            *self.iso_path.borrow() != String::from(LABEL_ISO_NONE) {
                                self.btn_valider.set_sensitive(true);
                                self.is_data_valide = true;
                                return true;
                        }
                    }
                }
            }
        }
        self.btn_valider.set_sensitive(false);
        self.is_data_valide = false;
        false
    }
    pub fn show(&self) {self.window.show();}
    //getter
    pub fn window(&self) -> ApplicationWindow { self.window.clone() }
    //getter
    pub fn iso_path(&self) -> String { self.iso_path.borrow().clone() }
    pub fn bs_value(&self) -> String { self.bs_value.borrow().clone() }
    pub fn disk_path(&self) -> String { self.disk_path.borrow().clone() }

    // setter
    fn set_field(&mut self,field: GuiField){
        match field {
            GuiField::IsoPath(path) => *self.iso_path.borrow_mut() = path,
            GuiField::DiskPath(path) => *self.disk_path.borrow_mut() = path,
            GuiField::BsValue(value) => *self.bs_value.borrow_mut() = value,
        }
    }
}


