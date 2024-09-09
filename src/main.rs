mod user_jobs;
use user_jobs::*;
use directories::ProjectDirs;
use serde_yaml_ng::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    println!("Hello, world!");

    let data_dir = ProjectDirs::from("com", "mycrom", "mycron").unwrap();
    let params_1 = JobParams {path: String::from("ls ~/Documents")};
    let job_1 = Job {name: String::from("List Documents"), timing: String::from("* * * * * * *"), params: params_1};

    let params_2 = JobParams {path: String::from("ls ~/videos")};
    let job_2 = Job {name: String::from("List Videos"), timing: String::from("*******"), params: params_2};

    let job_list = JobList {jobs: vec![job_1, job_2]};

    //println!("The job is: {:?}", &job_1);

    let mut file_path = PathBuf::from(data_dir.data_dir());
    //does the directory exist
    if !file_path.exists(){
        fs::create_dir_all(&file_path).unwrap();
    }
    file_path.push("list.yaml");
    if !file_path.exists(){
        File::create(&file_path).unwrap();
    }
    //let mut f = File::open(&file_path).unwrap();

    let output = serde_yaml_ng::to_string(&job_list).unwrap();
    println!("The yaml is: {:?}", output);
    fs::write(&file_path, &output).unwrap();
    println!("written to: {:?}", &file_path);
}
