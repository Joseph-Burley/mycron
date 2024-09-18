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
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct NewJob {
    #[arg(short, long)]
    name: String,

    #[arg(long)]
    minute: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Clisub {
    Edit(EditJob),
    New(NewJob),
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello world");
    let args = Args::parse();

    match args.subcommand {
        Clisub::Edit(j) => {
            println!("Editing a job: {:?}", j);
            let jl = load_from_file()?;
            let job = jl.find_name(&j.name);
            match job {
                None => println!("The job {} was not found", j.name),
                Some(j) => println!("Editing job")
            };

        },
        Clisub::New(j) => {
            println!("Creating a new job: {:?}", j);
        }
    }

    println!("loading jobs from file");
    let job_list = load_from_file()?;
    println!("{:?}", job_list);
    
    Ok(())
}