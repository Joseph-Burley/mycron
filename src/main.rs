extern crate mycron;
use mycron::user_jobs::*;

fn main() {
    println!("Hello, world!");

    let params_1 = JobParams {path: String::from("ls ~/Documents")};
    let job_1 = Job {name: String::from("List Documents"), timing: String::from("* * * * * * *"), params: params_1};

    println!("The job is: {:?}", job_1);
}
