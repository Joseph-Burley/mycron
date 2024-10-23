mod user_jobs;
mod cron_tab_wrapper;
mod settings;
extern crate simplelog;
use cron_tab_wrapper::create_job;
use settings::Settings;
use user_jobs::*;
use directories::ProjectDirs;
use std::fs::{self, File};
use std::path::PathBuf;
use std::sync::mpsc;
use std::process;
use chrono:: Utc;
use mycron::file_watcher::start_watch;
use mycron::Signal;
use simplelog::*;
#[macro_use] extern crate log;

fn main() {
    println!("The current process is: {}", process::id());

    let data_dir = ProjectDirs::from("com", "mycron", "mycron").unwrap();
    let mut pid_path = PathBuf::from(data_dir.data_dir());
    pid_path.push(".pid");
    if pid_path.exists() {
        let pid = fs::read_to_string(pid_path).unwrap();
        println!("Process is already running on pid: {}", pid);
        return;
    }
    let _ = fs::write(&pid_path, format!("{}\n", process::id()));

    //load system settings
    let system_settings = match Settings::load_settings(){
        Ok(s) => s,
        Err(e) => {
            println!("Error loading settings. Using default. {:?}", e);
            Settings::default()
        }
    };

    //set up logging
    let log_path = system_settings.get_system_log();
    let log_file = File::options().append(true).create(true).open(log_path).unwrap();
    let config = ConfigBuilder::default()
        .set_time_format_rfc3339()
        .set_time_level(LevelFilter::Error)
        .build();
    let _ = WriteLogger::init(LevelFilter::Debug, config, log_file).unwrap();

    debug!("system settings: {:?}", &system_settings);

    //load job lists
    let mut file_path = PathBuf::from(data_dir.data_dir());
    //does the directory exist
    if !file_path.exists(){
        fs::create_dir_all(&file_path).unwrap();
    }
    file_path.push("list.yaml");
    if !file_path.exists(){
        info!("List file does not exist. Creating blank file");
        File::create(&file_path).unwrap();
        let empty_job_list = JobList::default();
        let output = serde_yaml_ng::to_string(&empty_job_list).unwrap();
        fs::write(&file_path, &output).unwrap();
    }
    //TODO create file with no jobs if none exists DONE
    //TODO check if mycron is already running DONE
    //TODO allow for multiple list files (working on it)
    //TODO add default log location for jobs (mycronmanage?)
    //TODO change log format to use date-time (set_time_level) DONE
    //TODO use enums for thread signaling instead of integers DONE


    let (tx, rx) = mpsc::channel::<Signal>();
    let other_tx = tx.clone();
    start_watch(&file_path, tx);

    let mut cron = cron_tab::Cron::new(Utc);
    let mut job_handles: Vec<usize> = Vec::new();
    let mut should_continue = true;
    ctrlc::set_handler(move || {
        other_tx.send(Signal::Stop).unwrap();
        let _ = fs::remove_file(&pid_path);
    })
    .expect("Error setting Ctrl-C handler");

    while should_continue {
        println!("Starting cron");

        debug!("started reading from file");
        //read from file
        let input = fs::read_to_string(&file_path).unwrap();
        let new_jobs: JobList = serde_yaml_ng::from_str(&input).unwrap();
        debug!("Job count: {}", new_jobs.jobs.len());
        debug!("creating crontabs");
        
        //TODO maybe add default log location?
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
                println!("got signal: {:?}", val);
                match val {
                    Signal::Reload => {
                        info!("reloading cron");
                        cron.stop();
                        debug!("removing jogs");
                        for i in job_handles.drain(..) {
                            debug!("removing job: {}", i);
                            cron.remove(i);
                        }
                    },
                    Signal::Stop => {
                        info!("stopping");
                        should_continue = false;
                    }
                }
            },
            Err(e) => {
                println!("got error: {:?}", e);
            }
        }
    }
    cron.stop();
}
