mod user_jobs;
mod settings;
use std::error::Error;
use std::result::Result;
use clap::*;
use settings::Settings;
use user_jobs::*;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;


//Structs for parser
///Edit a job
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct EditJob {
    ///The name of the job to edit
    #[arg(short, long)]
    name: String,

    ///Set the minute argument of the cron expression
    #[arg(long)]
    minute: Option<String>,
    ///Set the hour argument of the cron expression
    #[arg(long)]
    hour: Option<String>,
    ///Set the day-of-week argument of the cron expression
    #[arg(long)]
    dow: Option<String>,
    ///Set the day-of-month argument of the cron expression
    #[arg(long)]
    dom: Option<String>,
    ///Set the month argument of the cron expression
    #[arg(long)]
    month: Option<String>,

    ///The command to run
    #[arg(short, long)]
    command: Option<String>,

    //todo: The best way to do this is probably to take a single string and slice it later.
    //trying to have clap do it runs into issues with flags.
    #[arg(short, long, value_delimiter=',', value_terminator=";", num_args=1..)]
    setargs: Vec<String>,

    ///Set the output location. Use "default" to use the system default.
    #[arg(short, long)]
    log: Option<String>
}

///Create a new job
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct NewJob {
    ///The name of the new job
    #[arg(short, long)]
    name: String,

    ///Set the minute argument of the cron expression
    #[arg(long)]
    minute: Option<String>,
    ///Set the hour argument of the cron expression
    #[arg(long)]
    hour: Option<String>,
    ///Set the day-of-week argument of the cron expression
    #[arg(long)]
    dow: Option<String>,
    ///Set the day-of-month argument of the cron expression
    #[arg(long)]
    dom: Option<String>,
    ///Set the month argument of the cron expression
    #[arg(long)]
    month: Option<String>,

    ///Set the command to run
    #[arg(short, long)]
    command: Option<String>,

    ///Set the output location
    #[arg(short, long)]
    log: Option<String>,
}

///Remove a job
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct RemoveJob {
    ///Name of job to remove
    #[arg(short, long)]
    name: String
}

///Change the default settings used by mycron
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ChangeSettings {
    #[arg(short, long)]
    syslog: Option<String>,
    #[arg(short, long)]
    joblog: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Clisub {
    Edit(EditJob),
    New(NewJob),
    Remove(RemoveJob),
    Settings(ChangeSettings),
    ///List mycron jobs
    List,
}

///mycronmanage - Add and edit jobs run by mycron and edit mycron settings
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
    let args = Args::parse();

    if !check_file()? {
        create_blank_file()?;
    }

    //if load settings fails (probably because the file doesn't exist) create it.
    let mut system_settings = Settings::load_settings().or(Settings::create_settings()).unwrap();

    //TODO allow editing a job to use the default log location
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

                    if j.dow.is_some() {
                        actualjob.timing.set_dow(j.dow.unwrap());
                    }

                    if j.dom.is_some() {
                        actualjob.timing.set_dom(j.dom.unwrap());
                    }

                    if j.month.is_some() {
                        actualjob.timing.set_month(j.month.unwrap());
                    }

                    if j.command.is_some() {
                        actualjob.params.command = j.command.unwrap();
                    }

                    if j.setargs.len() > 0 {
                        actualjob.params.arguments = j.setargs;
                    }

                    if j.log.is_some() {
                        let val = j.log.unwrap();
                        let p: PathBuf = if val.eq_ignore_ascii_case("default") {
                            println!("using default log location: {}", system_settings.get_job_log());
                            PathBuf::from(system_settings.get_job_log())
                        } else {
                            PathBuf::from(val)
                        };
                        if !p.exists() {
                            fs::File::create_new(&p).unwrap();
                        }
                        actualjob.params.set_log(&p);
                    }

                    write_to_file(jl)?;
                }
            };

        },
        Clisub::New(j) => {
            println!("Creating a new job: {:?}", j);
            let mut jl = load_from_file()?;
            let mut new_job = Job::new(&j.name);

            if j.minute.is_some() {
                new_job.timing.set_minute(j.minute.unwrap());
            }
            
            if j.hour.is_some() {
                new_job.timing.set_hour(j.hour.unwrap());
            }

            if j.dow.is_some() {
                new_job.timing.set_dow(j.dow.unwrap());
            }

            if j.dom.is_some() {
                new_job.timing.set_dom(j.dom.unwrap());
            }

            if j.month.is_some() {
                new_job.timing.set_month(j.month.unwrap());
            }

            if j.command.is_some() {
                new_job.params.command = j.command.unwrap();
            }

            let log_path = match j.log {
                Some(l) => {
                    let p = PathBuf::from(l);
                    if !p.exists() {
                        fs::File::create_new(&p).unwrap();
                    }
                    p
                },
                None => {
                    let mut p = PathBuf::from(system_settings.get_job_log());
                    p.push(format!("{}.log", new_job.name));
                    p
                }
            };
            new_job.params.set_log(&log_path);

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
        Clisub::Settings(s) =>
        {
            println!("Editing settings: {:?}", s);
            let mut no_error = true;
            /*
            let mut current_setting = Settings::load_settings().unwrap_or_default();
            if s.syslog.is_some() {
                let new_log = PathBuf::from(s.syslog.unwrap());
                match current_setting.set_system_log(&new_log) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Setting system log failed: {}", e);
                        no_error = false;
                    }
                }
            }
            */

            if s.joblog.is_some() {
                let new_log = PathBuf::from(s.joblog.unwrap());
                match system_settings.set_job_log(&new_log) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("Setting job log location failed: {}", e);
                        no_error = false;
                    }
                }
            }

            if no_error {
                Settings::save_settings(&system_settings).unwrap();
            } else {
                println!("Errors encountered while applying settings. Settings not changed");
            }
            
        },
        Clisub::List => {
            let job_list = load_from_file()?;

            let name_list: Vec<String> = job_list.jobs.iter().map(
                |x| format!("{}", x)).collect();
            println!("List of jobs:\n\t{}", name_list.join(",\n\t"));
        },
    }

    Ok(())
}