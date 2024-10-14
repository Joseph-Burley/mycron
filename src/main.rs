mod user_jobs;
mod cron_tab_wrapper;
extern crate simplelog;
use cron_tab_wrapper::create_job;
use user_jobs::*;
use directories::ProjectDirs;
use std::fs::{self, File};
use std::path::PathBuf;
use std::thread;
use chrono::{DateTime, Local, Utc};
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
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

    debug!("started reading from file");
    //read from file
    let input = fs::read_to_string(&file_path).unwrap();
    let new_jobs: JobList = serde_yaml_ng::from_str(&input).unwrap();
    debug!("Job count: {}", new_jobs.jobs.len());
    debug!("creating crontabs");
    let mut cron = cron_tab::Cron::new(Utc);
    for j in new_jobs.jobs {
        create_job(j, &mut cron);
    }

    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => {
                println!("Event detected: {:?}", event);
            },
            Err(e) => {
                println!("Error resolving event: {:?}", e);
            }
        }
    }).unwrap();

    //currently this kind of works. the watch handler will need to manually filter out events and files it doesn't need.
    //maybe move all this into a separate file for easy use?
    let mut watch_file = file_path.clone();
    watch_file.pop(); //this will make the watcher watch every file in the directory
    let watch_thread = thread::spawn(move || {
        watcher.watch(&watch_file, RecursiveMode::Recursive).unwrap();
        thread::park();
    });
    cron.start();
    watch_thread.join().unwrap();
    //std::thread::park();
    //std::thread::sleep(std::time::Duration::from_secs(20));
    //cron.stop();
}
