mod user_jobs;
mod cron_tab_wrapper;
extern crate simplelog;
use cron_tab_wrapper::{create_job, init_cron, stop_cron};
use user_jobs::*;
use directories::ProjectDirs;
use std::fs::{self, File};
use std::path::PathBuf;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use chrono::{DateTime, Local, Utc};
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use mycron::file_watcher::{self, start_watch};
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


    let (tx, rx) = mpsc::channel::<u32>();

    let rx_thread = thread::spawn(|| {
        
    });

    start_watch(&file_path, tx);
    let cron_thread = init_cron(&file_path);

    loop {
        let rec = rx.recv_timeout(Duration::from_millis(10));
        match rec {
            Ok(val) => {
                println!("got number: {}", val);
                if val == 42 {
                    println!("stopping");
                    stop_cron(&cron_thread);
                }
            },
            Err(e) => {
                //println!("got erroe: {:?}", e);
            }
        }
        std::thread::sleep(Duration::from_secs(1));
    }
    
    std::thread::park();

}
