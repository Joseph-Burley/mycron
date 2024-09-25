use std::error::Error;
//use core::marker::Send;
use cron_tab::*;
use chrono::{Offset, TimeZone, Utc};
//use chrono::offset::Offset;
//use chrono::offset::TimeZone::Offset;
use crate::Job;

//let utc_tz = Utc;


pub fn create_job(job: Job, cron: &mut Cron<Utc>) {
    cron.add_fn(&job.timing.full_timing(), move || execute_job(&job.params.path)).unwrap();
}

/*
pub fn create_job<T>(job: Job, cron: &mut Cron<T>) where
T: TimeZone + Send + Sync + 'static,
T::Offset: Send, {
    cron.add_fn(&job.timing, move || execute_job(&job.params.path)).unwrap();
}
*/


/*
pub fn create_job (job: Job) -> Result<Cron<Utc>> {
    let mut c = Cron::new(Utc);
    c.add_fn(job.timing.as_str(), move || execute_job(&job.params.path))?;
    return Result::Err(cron_tab::CronError::Unknown);
    //return Result::Err("This is supposed to fail");
}
    */

fn execute_job(command: &str) {
    println!("The command is: {}", command);
}