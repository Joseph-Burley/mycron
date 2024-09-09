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

#[derive(Debug, Serialize,Deserialize)]
pub struct JobList {
    pub jobs: Vec<Job>,
}
