use std::{fmt::{self}, path::Path};
use serde::{Serialize, Deserialize};
use getset::{Getters, Setters};
use directories::{UserDirs, ProjectDirs};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobParams {
    pub command: String,
    pub arguments: Vec<String>,
    pub log_location: String
}

impl Default for JobParams {
    fn default() -> Self {
        let udir = UserDirs::new().unwrap();
        let mut def_log = PathBuf::from(udir.home_dir());
        def_log.push("mycron_logs/default.log");
        JobParams {
            command: String::from(""),
            arguments: Vec::<String>::new(),
            log_location: String::from(def_log.to_str().unwrap()),
        }
    }
}

impl JobParams {
    pub fn full_command(&self) -> String {
        format!("{} {}\n\tLog Location: {}", self.command, self.arguments.join(" "), self.log_location)
    }

    pub fn set_log(&mut self, p: &Path) {
        self.log_location = String::from(p.to_str().unwrap());
    }
}

#[derive(Debug, Serialize, Deserialize, Getters, Setters)]
pub struct Timing {
    #[getset(get = "pub", set = "pub")]
    second: String,

    #[getset(get = "pub", set = "pub")]
    minute: String,

    #[getset(get = "pub", set = "pub")]
    hour: String,

    #[getset(get = "pub", set = "pub")]
    dom: String,

    #[getset(get = "pub", set = "pub")]
    dow: String,

    #[getset(get = "pub", set = "pub")]
    month: String,

    #[getset(get = "pub", set = "pub")]
    year: String,
}

impl Default for Timing {
    fn default() -> Self {
        Timing {
            second: String::from("0"),
            minute: String::from("*"),
            hour: String::from("*"),
            dom: String::from("*"),
            dow: String::from("*"),
            month: String::from("*"),
            year: String::from("*"),
        }
    }
}

impl fmt::Display for Timing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {} {}", self.minute, self.hour, self.dom, self.dow, self.month)
    } 
}


impl Timing {
    pub fn full_timing(&self) -> String {
        format!("{} {} {} {} {} {} {}", self.second,
                                        self.minute,
                                        self.hour,
                                        self.dom,
                                        self.dow,
                                        self.month,
                                        self.year)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub timing: Timing,
    pub params: JobParams,
}

impl Job {
    pub fn new(n: &str) -> Job {
        Job {
            name: n.to_string(),
            timing: Timing::default(),
            params: JobParams::default()
        }
    }
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} {}", self.name, self.timing, self.params.full_command())
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobList {
    pub jobs: Vec<Job>,
}

impl JobList {
    pub fn find_name(&self, n: &str) -> Option<&Job>{
        self.jobs.iter().find(|&i| i.name.eq(n))
    }

    pub fn find_name_mut(&mut self, n: &str) -> Option<&mut Job>{
        self.jobs.iter_mut().find(|i| i.name.eq(n))
        
    }

    pub fn find_name_index(&self, n: &str) -> Option<usize> {
        self.jobs.iter().position(|i| i.name.eq(n))
    }
}
