mod user_jobs;
use std::error::Error;
use std::result::Result;
use clap::*;
use user_jobs::*;
use directories::ProjectDirs;
use serde_yaml_ng::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};


//Structs for parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct EditJob {
    #[arg(short, long)]
    name: String,

    #[arg(long)]
    minute: Option<String>,
    #[arg(long)]
    hour: Option<String>,

    #[arg(short, long)]
    command: Option<String>,

    //this should allow for multiple arguments, but does not for some reason.
    #[arg(short, long)]
    setargs: Vec<String>
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct NewJob {
    #[arg(short, long)]
    name: String,

    #[arg(long)]
    minute: Option<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RemoveJob {
    #[arg(short, long)]
    name: String
}

#[derive(Debug, Subcommand)]
enum Clisub {
    Edit(EditJob),
    New(NewJob),
    Remove(RemoveJob),
    List,
}

#[derive(Parser, Debug)]
struct Args{
    #[command(subcommand)]
    subcommand: Clisub,
}
//--------------------------------------

fn load_from_file() -> Result<JobList, Box<dyn Error>> {
    let data_dir = match ProjectDirs::from("com", "mycron", "mycron") {
        None => {return Result::Err(String::from("Could not find project directory").into())},
        Some(f) => f
    };
    let mut file = PathBuf::from(data_dir.data_dir());
    file.push("list.yaml");
    let input_string = fs::read_to_string(&file)?;
    let jobs: JobList = serde_yaml_ng::from_str(&input_string)?;
    return Result::Ok(jobs);
}

fn write_to_file(jl: JobList) -> Result<(), Box<dyn Error>> {
    let data_dir = match ProjectDirs::from("com", "mycron", "mycron") {
        None => {return Result::Err(String::from("Could not find project directory").into())},
        Some(f) => f
    };
    let mut file = PathBuf::from(data_dir.data_dir());
    file.push("list.yaml");
    let output = serde_yaml_ng::to_string(&jl)?;
    fs::write(&file, &output)?;
    Ok(())
}

fn check_file() -> Result<bool, Box<dyn Error>> {
    let data_dir = match ProjectDirs::from("com", "mycron", "mycron") {
        None => {return Result::Err(String::from("Could not find project directory").into())},
        Some(f) => f
    };
    let mut file = PathBuf::from(data_dir.data_dir());
    file.push("list.yaml");
    return Ok(file.exists());
}

fn create_blank_file() -> Result<(), Box<dyn Error>> {
    let data_dir = match ProjectDirs::from("com", "mycron", "mycron") {
        None => {return Result::Err(String::from("Could not find project directory").into())},
        Some(f) => f
    };
    let mut file = PathBuf::from(data_dir.data_dir());
    file.push("list.yaml");
    let job_list = JobList::default();
    fs::write(file, "")?;
    write_to_file(job_list)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello world");
    let args = Args::parse();

    if !check_file()? {
        create_blank_file()?;
    }

    match args.subcommand {
        Clisub::Edit(j) => {
            println!("Editing a job: {:?}", j);
            let mut jl = load_from_file()?;
            let job = jl.find_name_mut(&j.name);
            match job {
                None => println!("The job {} was not found", j.name),
                Some(actualjob) => {
                    if j.minute.is_some() {
                        actualjob.timing.set_minute(j.minute.unwrap());
                    }

                    if j.hour.is_some() {
                        actualjob.timing.set_hour(j.hour.unwrap());
                    }

                    if j.command.is_some() {
                        actualjob.params.command = j.command.unwrap();
                    }

                    if j.setargs.len() > 0 {
                        actualjob.params.arguments = j.setargs;
                    }

                    write_to_file(jl)?;
                }
            };

        },
        Clisub::New(j) => {
            println!("Creating a new job: {:?}", j);
            let mut jl = load_from_file()?;
            let new_job = Job::new(&j.name);
            jl.jobs.push(new_job);
            write_to_file(jl)?;
        },
        Clisub::Remove(j) => {
            let mut jl = load_from_file()?;
            let job_index = jl.find_name_index(&j.name);
            match job_index {
                None => println!("Job not found in list"),
                Some(i) => {
                    jl.jobs.remove(i);
                }
            }
            write_to_file(jl)?;
        },
        Clisub::List => {
            let job_list = load_from_file()?;

            let name_list: Vec<String> = job_list.jobs.iter().map(
                |x| format!("{}", x)).collect();
            println!("List of jobs:\n\t{}", name_list.join(",\n\t"));
        },
    }

    /*
    println!("loading jobs from file");
    let job_list = load_from_file()?;
    println!("{:?}", job_list);
    */
    Ok(())
}