use clap::{Parser, Subcommand};
use std::env;
use std::path::Path;

use maid_lang::{
    create_package_dir,
    new_project,
    add_package,
    remove_package,
    update_package,
    run,
    launch_repl,
};

const VERSION: &str = "2.6";

#[derive(Parser)]
#[command(name = "maid", version = VERSION, about = "The MaidCode Programming Language")]
struct Cli {
    file: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create a maid project")]
    New { name: String },
    #[command(about = "Initialize a maid project in the current directory")]
    Init,
    #[command(about = "Install a maid kennel from the internet")]
    Install { name: String },
    #[command(about = "Remove an installed maid kennel")]
    Remove { name: String },
    #[command(about = "Update an installed maid kennel to the latest version")]
    Update { name: String },
}

fn main() {
    unsafe {
        let std_path = env::current_exe()
            .expect("Unable to retrieve current exe")
            .parent()
            .unwrap()
            .join("library")
            .to_string_lossy()
            .replace("\\", "/")
            .replace("target/debug/", "")
            .replace("target/release/", "");

        let pkg_path = env::current_exe()
            .expect("Unable to retrieve current exe")
            .parent()
            .unwrap()
            .join("kennels")
            .to_string_lossy()
            .replace("\\", "/")
            .replace("target/debug/", "")
            .replace("target/release/", "");

        env::set_var("MAID_STD", &std_path);
        env::set_var("MAID_PKG", &pkg_path);
    }

    crate::create_package_dir();

    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::New { name }), _) => {
            crate::new_project(Path::new(&name), false);
        }
        (Some(Commands::Init), _) => {
            crate::new_project(Path::new("."), true);
        }
        (Some(Commands::Install { name }), _) => {
            crate::add_package(&name);
        }
        (Some(Commands::Remove { name }), _) => {
            crate::remove_package(&name);
        }
        (Some(Commands::Update { name }), _) => {
            crate::update_package(&name);
        }
        (None, Some(file)) => {
            let error = crate::run(&file, None);

            if let Some(err) = error {
                println!("{err}");
            }
        }
        (None, None) => {
            crate::launch_repl(VERSION);
        }
    }
}
