mod user_jobs;
mod cron_tab_wrapper;
use cron_tab_wrapper::create_job;
use user_jobs::*;
use directories::ProjectDirs;
use std::fs::{self, File};
use std::path::PathBuf;
use chrono::Utc;

fn main() {
    println!("Hello, world!");

    let data_dir = ProjectDirs::from("com", "mycron", "mycron").unwrap();
    let mut file_path = PathBuf::from(data_dir.data_dir());
    //does the directory exist
    if !file_path.exists(){
        fs::create_dir_all(&file_path).unwrap();
    }
    file_path.push("list.yaml");
    if !file_path.exists(){
        File::create(&file_path).unwrap();
    }

    //read from file
    let input = fs::read_to_string(&file_path).unwrap();
    let new_jobs: JobList = serde_yaml_ng::from_str(&input).unwrap();
    println!("The jobs from the file:\n{:?}", new_jobs);
    println!("The first job:\n{:?}", new_jobs.jobs[0]);

    println!("creating crontabs");
    let mut cron = cron_tab::Cron::new(Utc);
    for j in new_jobs.jobs {
        create_job(j, &mut cron);
    }

    cron.start();
    std::thread::park();
    //std::thread::sleep(std::time::Duration::from_secs(20));
    //cron.stop();
}
