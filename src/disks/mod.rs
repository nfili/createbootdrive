use std::fmt::Display;
use std::fs;
use std::path::Path;
use crate::utilitaires::human_read;

#[derive(Default,Debug,Clone)]
pub struct Disk {
    id: i8,
    device: String,
    mount_point: String,
    pub(crate) size: u64,
    size_human: String,
}
#[derive(Default, Debug, Clone)]
pub struct Disks {
    disk: Vec<Disk>,
}

impl Disk{
    fn default() -> Self {
        Disk {
            id:0,
            device :String::new(),
            mount_point: String::new(),
            size: 0,
            size_human: String::new(),
        }
    }
    fn new() -> Self {
        Self::default()

    }
    fn set_disk(&mut self,id: i8, device: String, mount_point: String, size: u64){
        self.id=id;
        self.device=device;
        self.mount_point = mount_point;
        self.size = size;
        self.with_human_size();
    }
    fn with_human_size(&mut self){
        self.size_human = human_read(self.size);
    }

}
impl Disks {
    fn default() -> Self {
        Disks {
            disk: Vec::new(),
        }
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_disks(&mut self){
        self.disk.clear();
        let mut disk = Disk::new();
        let list_disk = Self::get_external_drive();
        list_disk.iter().enumerate().for_each(|(i,s)| {
            let path = Path::new(s);
            let mount = Self::get_mount_point(s.to_string());
            let mount_rpl = mount.replace("\\040", " ");
            let size = Self::get_total_space_disk(path);
            disk.set_disk(i as i8, s.to_string(), mount_rpl, size, /* std::string::String */);
            self.disk.push(disk.clone());
        });
    }
    fn get_external_drive() -> Vec<String> {
        //variable à retourner : liste des disques externe USB
        let mut list_disque = Vec::new();
        // Création du recenseur pour udev
        let mut enumerator = udev::Enumerator::new().unwrap();

        // Ajout des filtres pour trouver les disques USB
        enumerator.match_subsystem("block").unwrap();
        enumerator.match_attribute("removable", "1").unwrap(); // Amovible

        // Parcours tous les périphériques qui répondent positif aux filtres
        for device in enumerator.scan_devices().unwrap() {
            // Récuperation du chemin du nœud du périphérique
            let devnode = device.devnode().unwrap_or(Path::new(""));
            // Si le chemin du nœud existe, on l'ajoute à la liste des disques
            if devnode.exists(){
                list_disque.push(String::from(devnode.to_str().unwrap()));
            }

        }
        // On retourne la liste
        list_disque
    }
    fn get_mount_point(path: String) -> String {
        if let Ok(contents) = fs::read_to_string("/proc/partitions") {
            for line in contents.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let device = parts[0].to_string();    // 1er champ : périphérique
                    if device.contains(path.as_str()) {
                        println!("device : {}", device);
                        return parts[1].to_string();
                    }
                }
            }
        };
        String::new()
    }
    fn get_total_space_disk(path: &Path) -> u64 {
        if let Some(parts_path) = path.file_name().and_then(|os_str|os_str.to_str()){
            if let Ok(contents) = fs::read_to_string("/proc/partitions") {
                for line in contents.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        let device = parts[3].to_string();    // 1er champ : périphérique
                        if device == parts_path {
                            let size = parts[2].parse::<u64>().unwrap_or(0);
                            return size*1024;
                        }
                    }
                }
            }
        }
        0
    }
    pub fn disks(&self) -> Vec<Disk> {
        self.disk.clone()
    }
    //getter
    pub fn get_disk_by_device(&self, device: &str) -> Option<&Disk> {
        self.disk.iter().find(|d| d.device == device)
    }
}
impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.device, self.size_human)
    }
}