use std::path::PathBuf;
use directories::ProjectDirs;

pub fn get_daemon_log_path() -> Result<PathBuf, &'static str> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "floffah", "mcproc") {
        let log_dir = proj_dirs.data_dir().join("logs");

        if !log_dir.exists() {
            std::fs::create_dir_all(log_dir.clone()).expect("Failed to create log directory")
        }

        // file path that the name is the current time
        let log_file = log_dir.join(format!("{}.log", chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")));

        return Ok(log_file);
    }

    return Err("Failed to get daemon log path");
}