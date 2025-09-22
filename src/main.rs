use clap::{Parser, Subcommand};
use std::process::Command;
use std::{env, fs, path::Path};
mod init;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short = 'l', long = "lang")]
        lang: String,
    },
    Run {},
    Build {},
    Clean {},
    New {
        #[arg(short = 'l', long = "lang")]
        lang: String,
        project_name: String,
    },
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init { lang }) => {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let folder_name = current_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("<unknown>");
            init::setup(lang.to_string(), folder_name.to_string());
        }
        Some(Commands::Run {}) => {
            run();
        }
        Some(Commands::Build {}) => {
            build();
        }
        Some(Commands::Clean {}) => {
            clean();
        }
        Some(Commands::New { lang, project_name }) => {
            new(lang.to_string(), project_name.to_string());
        }
        None => {}
    }
}

fn run() {
    let _ = Command::new("make")
        .arg("run")
        .status()
        .expect("Failed to execute make run");
}

fn build() {
    let _ = Command::new("make")
        .arg("build")
        .status()
        .expect("Failed to execute make run");
}

fn clean() {
    let _ = Command::new("make")
        .arg("clean")
        .status()
        .expect("Failed to execute make run");
}
fn new(cmdlang: String, project_name: String) {
    let path = Path::new(&project_name);

    if path.exists() {
        eprintln!("Error: Directory '{}' already exists!", project_name);
        std::process::exit(1);
    }

    fs::create_dir(path).expect("Failed to create project folder");

    env::set_current_dir(path).expect("Failed to enter project folder");

    init::setup(cmdlang, project_name);
}
