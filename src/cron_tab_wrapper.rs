use cron_tab::*;
use chrono::Utc;
use crate::{Job, JobParams};
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

pub fn create_job(job: Job, cron: &mut Cron<Utc>, default_log_loc: &String) -> Result<usize> { //or cron error
    let output_loc: PathBuf = if !job.params.log_location.is_empty() {
        PathBuf::from(&job.params.log_location)
    } else {
        PathBuf::from(default_log_loc).join(&job.name)
    };
    if !output_loc.exists() {
        fs::write(&output_loc, "").unwrap();
    }
    let h = cron.add_fn(&job.timing.full_timing(), 
        move || execute_job(&job.name, &job.params, &output_loc).map_err(|e| {
            error!("Error when execution a job: {:?}", e);
            e
        }).expect("something went wrong executing a job")).unwrap();
    debug!("Adding job: {}", h);
    Ok(h)
}


fn execute_job(name: &String, params: &JobParams, output_loc: &PathBuf) -> core::result::Result<(), Box<dyn Error>> {
    info!("Running: {}", name);
    debug!("command: {}", params.command);
    let output = Command::new("sh")
                                .arg("-c")
                                .arg(&params.command)
                                .output()
                                .map_err(|e| {
                                    error!("Error when running command: {:?}", e);
                                    e
                                })?;
    //TODO rewrite this to use log_append from params
    //this may require using std::io in addition to std::fs
    fs::write(&output_loc, output.stdout).map_err(|e| {
        error!("Could not write to log location: {:?}", e);
        e
    })?;

    if output.stderr.len() > 0 {
        info!("The job \"{}\" encountered and error", name);
        let mut error_path = PathBuf::from(&output_loc);
        error_path.pop();
        error_path.push("error.log");
        info!("stderr written to: {}", error_path.display());
        fs::write(error_path, output.stderr).map_err(|e| {
            error!("Could not write to error location: {:?}", e);
            e
        })?;
    }
    
    Ok(())
}
