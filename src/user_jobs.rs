use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JobParams {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub name: String,
    pub timing: String,
    pub params: JobParams,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobList {
    pub jobs: Vec<Job>,
}

impl JobList {
    pub fn find_name(&self, n: &str) -> Option<&Job>{
        self.jobs.iter().find(|&i| i.name.eq(n))
    }
}
