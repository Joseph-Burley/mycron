use std::fmt::{self};

use serde::{Serialize, Deserialize};
use getset::{Getters, Setters};

#[derive(Debug, Serialize, Deserialize)]
pub struct JobParams {
    pub command: String,
    pub arguments: Vec<String>,
}

impl Default for JobParams {
    fn default() -> Self {
        JobParams {
            command: String::from(""),
            arguments: Vec::<String>::new(),
        }
    }
}

impl JobParams {
    pub fn full_command(&self) -> String {
        format!("{} {}", self.command, self.arguments.join(" "))
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
