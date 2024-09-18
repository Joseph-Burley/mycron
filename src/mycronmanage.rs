use clap::*;

/*
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(long)]
    minute: Option<String>,

    #[arg(long)]
    hour: Option<String>,

    #[arg(long)]
    dow: Option<String>,

    #[arg(long)]
    dom: Option<String>,

    #[arg(long)]
    month: Option<String>,
}
*/

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

fn main() {
    println!("Hello world");
    let args = Args::parse();

    match args.subcommand {
        Clisub::Edit(j) => {
            println!("Editing a job: {:?}", j);
        },
        Clisub::New(j) => {
            println!("Creating a new job: {:?}", j);
        }
    }
    
}