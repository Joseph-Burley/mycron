use cron_tab::*;
use chrono::Utc;
use crate::Job;

//let utc_tz = Utc;


pub fn create_job(job: Job, cron: &mut Cron<Utc>) {
    cron.add_fn(&job.timing.full_timing(), move || execute_job(&job.params.command)).unwrap();
}

fn execute_job(command: &str) {
    println!("The command is: {}", command);
}