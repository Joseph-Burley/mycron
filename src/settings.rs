use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::{default, fs};
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub system_log: String,
    pub job_log: String,
    pub job_file: String,
}

impl Default for Settings {
    fn default() -> Self {
        let data_dir = ProjectDirs::from("com", "mycron", "mycron").unwrap();
        let mut system_log_path = PathBuf::from(data_dir.data_dir());
        system_log_path.push("mycron_log.log");
        let mut job_log_path = PathBuf::from(data_dir.data_dir());
        job_log_path.push("job_logs");
        let mut def_job_file = PathBuf::from(data_dir.data_dir());
        def_job_file.push("list.yaml");

        Settings {
            system_log: system_log_path.into_os_string().into_string().unwrap(),
            job_log: job_log_path.into_os_string().into_string().unwrap(),
            job_file: def_job_file.into_os_string().into_string().unwrap()
        }
    }
}

impl Settings {
    //modify so functions actually use this
    pub fn get_base_dir() -> Result<String, Box<dyn Error>> {
        let data_dir = match ProjectDirs::from("com", "mycron", "mycron"){
            Some(v) => v,
            None => {
                return Result::Err(String::from("Cannot find settings file in project directory").into());
            }
        };
        let p = data_dir.data_dir().to_str().unwrap();
        Ok(String::from(p))
    }

    pub fn load_settings() -> Result<Settings, Box<dyn Error>> {
        let data_dir = match ProjectDirs::from("com", "mycron", "mycron"){
            Some(v) => v,
            None => {
                return Result::Err(String::from("Cannot find settings file in project directory").into());
            }
        };
        let mut settings_path = PathBuf::from(data_dir.data_dir());
        settings_path.push("settings.yaml");
        let input_string = fs::read_to_string(&settings_path)?;
        let s: Settings = serde_yaml_ng::from_str(&input_string)?;
        return Ok(s);
    }

    pub fn save_settings(s: &Settings) -> Result<(), Box<dyn Error>> {
        let data_dir = match ProjectDirs::from("com", "mycron", "mycron"){
            Some(v) => v,
            None => {
                return Result::Err(String::from("Cannot find settings file in project directory").into());
            }
        };
        let mut settings_path = PathBuf::from(data_dir.data_dir());
        settings_path.push("settings.yaml");
        let output_string = serde_yaml_ng::to_string(&s)?;
        fs::write(settings_path, &output_string)?;
        Ok(())
    }

    pub fn create_settings() -> Result<Settings, Box<dyn Error>> {
        let s = Settings::default();
        Settings::save_settings(&s)?;
        Ok(s)
    }

    pub fn set_system_log(&mut self, p: &Path) -> Result<(), String> {
        if p.exists() && p.is_file() {
            self.system_log = String::from(p.to_str().unwrap());
            return Ok(());
        } else {
            return Err(format!("Path: {} either is not valid or is not a file", p.to_str().unwrap()));
        }
    }

    pub fn get_system_log(&self) -> String {
        self.system_log.clone()
    }

    pub fn set_job_log(&mut self, p: &Path) -> Result<(), String> {
        if p.exists() && p.is_dir() {
            self.job_log = String::from(p.to_str().unwrap());
            return Ok(());
        } else {
            return Err(format!("Path: {} either is not valid or is not a directory", p.to_str().unwrap()));
        }
    }

    pub fn get_job_log(&self) -> String {
        self.job_log.clone()
    }

    pub fn set_job_file(&mut self, p: &Path) -> Result<(), String> {
        if p.exists() && p.is_file() {
            self.job_file = String::from(p.to_str().unwrap());
            return Ok(());
        } else {
            return Err(format!("Path: {} either is not valid or is not a file", p.to_str().unwrap()));
        }
    }

    pub fn get_job_file(&self) -> String {
        self.job_file.clone()
    }
}

//test commit just for my sanity