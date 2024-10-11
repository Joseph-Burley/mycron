use cron_tab::*;
use chrono::Utc;
use crate::{Job, JobParams};
use std::process::Command;
use std::fs;
use std::path::PathBuf;

//let utc_tz = Utc;


pub fn create_job(job: Job, cron: &mut Cron<Utc>) {
    cron.add_fn(&job.timing.full_timing(), move || execute_job(&job.params)).unwrap();
}

//TODO Add global log to capture errors not generated by user commands
//TODO Find a way to redirect errors
//TODO Append to output (optional?)
fn execute_job(params: &JobParams) {
    println!("The command is: {}", params.full_command());
    let output = Command::new("sh")
                                .arg("-c")
                                .arg(&params.command)
                                .output().expect("could not run program for some reason?");
    fs::write(&params.log_location, output.stdout).expect("Could not write to log location after executing");
    let mut error_path = PathBuf::from(&params.log_location);
    error_path.pop();
    error_path.push("error.log");
    println!("error path: {:?}", error_path);
    fs::write(error_path, output.stderr).expect("Could not write to error locaiton");
}