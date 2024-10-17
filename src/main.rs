mod user_jobs;
mod cron_tab_wrapper;
extern crate simplelog;
use cron_tab_wrapper::create_job;
use user_jobs::*;
use directories::ProjectDirs;
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::mpsc;
use chrono:: Utc;
use mycron::file_watcher::start_watch;
use simplelog::*;
#[macro_use] extern crate log;

fn main() {
    let data_dir = ProjectDirs::from("com", "mycron", "mycron").unwrap();
    let mut log_path = PathBuf::from(data_dir.data_dir());
    log_path.push("mycron_log.log");
    let log_file = File::options().append(true).create(true).open(log_path).unwrap();
    let _ = WriteLogger::init(LevelFilter::Debug, Config::default(), log_file).unwrap();
    let mut file_path = PathBuf::from(data_dir.data_dir());
    //does the directory exist
    if !file_path.exists(){
        fs::create_dir_all(&file_path).unwrap();
    }
    file_path.push("list.yaml");
    if !file_path.exists(){
        File::create(&file_path).unwrap();
    }
    //TODO create file with no jobs if none exists


    let (tx, rx) = mpsc::channel::<u32>();

    start_watch(&file_path, tx);
    

    let mut cron = cron_tab::Cron::new(Utc);
    let mut job_handles: Vec<usize> = Vec::new();
    loop {
        println!("Starting cron");

        debug!("started reading from file");
        //read from file
        let input = fs::read_to_string(&file_path).unwrap();
        let new_jobs: JobList = serde_yaml_ng::from_str(&input).unwrap();
        debug!("Job count: {}", new_jobs.jobs.len());
        debug!("creating crontabs");
        
        
        for j in new_jobs.jobs {
            match create_job(j, &mut cron) {
                Ok(h) => job_handles.push(h),
                Err(e) => {
                    error!("Got an error while adding a cronjob: {:?}", e);
                }
            }
        }

        cron.start();
        let rec = rx.recv();
        match rec {
            Ok(val) => {
                println!("got number: {}", val);
                if val == 42 {
                    println!("stopping");
                    info!("stopping cron");
                    cron.stop();
                    debug!("removing jobs");
                    for i in job_handles.drain(..) {
                        debug!("removing job: {}", i);
                        cron.remove(i);
                    }
                }
            },
            Err(e) => {
                println!("got erroe: {:?}", e);
            }
        }
    }
}
