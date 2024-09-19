use clap::builder::Str;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JobParams {
    pub path: String,
}

impl Default for JobParams {
    fn default() -> Self {
        JobParams {
            path: String::from("")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub timing: String,
    pub params: JobParams,
}

impl Job {
    pub fn new(n: &str) -> Job {
        Job {
            name: n.to_string(),
            timing: String::from("0 * * * * * *"),
            params: JobParams::default()
        }
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

    pub fn find_name_index(&self, n: &str) -> Option<usize> {
        self.jobs.iter().position(|i| i.name.eq(n))
    }
}
