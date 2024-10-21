use directories::ProjectDirs;
use std::path::PathBuf;

pub struct Settings {
    pub system_log: String,
    pub job_log: String,
    pub job_files: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        let data_dir = ProjectDirs::from("com", "mycron", "mycron").unwrap();
        let mut system_log_path = PathBuf::from(data_dir.data_dir());
        system_log_path.push("mycron_log.log");
        let mut job_log_path = PathBuf::from(data_dir.data_dir());
        job_log_path.push("job_logs");

        Settings {
            system_log: system_log_path.into_os_string().into_string().unwrap(),
            job_log: job_log_path.into_os_string().into_string().unwrap(),
            job_files: Vec::new()
        }
    }
}