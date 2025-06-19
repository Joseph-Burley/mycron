use cron_tab::*;
use chrono::Utc;
use crate::{Job, JobParams};
use std::io::Write;
use std::process::Command;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

pub fn create_job(job: Job, cron: &mut Cron<Utc>, default_log_loc: &String) -> Result<usize> { //or cron error
    let h = cron.add_fn(&job.timing.full_timing(), 
        move || execute_job(&job.name, &job.params).map_err(|e| {
            error!("Error when execution a job: {:?}", e);
            e
        }).expect("something went wrong executing a job")).unwrap();
    debug!("Adding job: {}", h);
    Ok(h)
}

//removing output_loc: &PathBuf
fn execute_job(name: &String, params: &JobParams) -> core::result::Result<(), Box<dyn Error>> {
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

    let mut file = match params.log_append {
        true => fs::OpenOptions::new().create(true).append(true).open(&params.log_location).unwrap(),
        false => fs::OpenOptions::new().create(true).write(true).truncate(true).open(&params.log_location).unwrap()
    };
    file.write(&output.stdout).map_err(|e| {
        error!("Could not write to log location: {:?}", e);
        e
    })?;

    if output.stderr.len() > 0 {
        info!("The job \"{}\" encountered an error", name);
        let mut error_path = PathBuf::from(&params.log_location);
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
