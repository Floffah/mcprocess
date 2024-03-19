use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GlobalData {
    pub servers: Vec<ServerData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServerStatus {
    Starting,
    Running,
    Stopped,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerData {
    pub name: String,
    pub path: PathBuf,
    pub startup_file: String,
    pub java_vm_options: String,
    pub status: ServerStatus,
    pub autostart: bool,
}

impl GlobalData {
    pub fn write(&self) -> Result<(), &'static str> {
        let global_data_file = get_global_data_path()?;

        let str = toml::to_string(&self).expect("Failed to serialize global data");
        std::fs::write(global_data_file, str).expect("Failed to write global data file");

        return Ok(());
    }
}

pub fn get_global_data_path() -> Result<PathBuf, &'static str> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "floffah", "mcproc") {
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            std::fs::create_dir_all(data_dir).expect("Failed to create data directory")
        }

        let global_data_file = data_dir.join("global_state.toml");

        println!("Global data file: {:?}", global_data_file);

        return Ok(global_data_file);
    }

    return Err("Failed to get global data path");
}

pub fn read_global_data() -> Result<GlobalData, &'static str> {
    let global_data_file = get_global_data_path()?;

    if !global_data_file.exists() {
        let initial_data = GlobalData { servers: vec![] };

        initial_data.write().expect("Failed to write global data file");

        return Ok(initial_data);
    }

    let str = std::fs::read_to_string(global_data_file).expect("Failed to read global data file");
    let data: GlobalData = toml::from_str(&str).expect("Failed to parse global data file");

    return Ok(data);
}