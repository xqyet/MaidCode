use clap::{Parser, Subcommand};
use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

use maid_lang::{
    create_package_dir, new_project, add_package, remove_package, update_package, run, launch_repl,
};

use include_dir::{include_dir, Dir};
static STD_DIR: Dir<'_>     = include_dir!("$CARGO_MANIFEST_DIR/library");
static KENNELS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/kennels");

const VERSION: &str = "2.6";

#[derive(Parser)]
#[command(name = "maid", version = VERSION, about = "The MaidCode Programming Language")]
struct Cli {
    /// Path to a .maid file to run
    file: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a maid project
    New { name: String },
    /// Initialize a maid project in the current directory
    Init,
    /// Install a maid kennel from the internet
    Install { name: String },
    /// Remove an installed maid kennel
    Remove { name: String },
    /// Update an installed maid kennel to the latest version
    Update { name: String },
}

/// Ensure stdlib + kennels are available and point MAID_STD / MAID_PKG to them.
fn ensure_std_available() -> (PathBuf, PathBuf) {
    // Respect explicit env overrides
    if let (Ok(std), Ok(pkg)) = (env::var("MAID_STD"), env::var("MAID_PKG")) {
        return (PathBuf::from(std), PathBuf::from(pkg));
    }

    // Dev mode: when running from the repo, use live files
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_std  = repo_root.join("library");
    let repo_pkg  = repo_root.join("kennels"); // kennels/kennels.maid
    if cfg!(debug_assertions) && repo_std.exists() {
        env::set_var("MAID_STD", &repo_std);
        env::set_var("MAID_PKG", &repo_pkg);
        return (repo_std, repo_pkg);
    }

    // Installed path (per-user data dir)
    let base     = dirs::data_local_dir().or_else(dirs::home_dir).unwrap().join("maid");
    let std_path = base.join("library");
    let pkg_path = base.join("kennels");

    // Create dirs first
    let _ = fs::create_dir_all(&std_path);
    let _ = fs::create_dir_all(&pkg_path);

    // Extract embedded assets (idempotent)
    if !STD_DIR.entries().is_empty() {
        let _ = STD_DIR.extract(&std_path);
    } else {
        eprintln!("Warning: embedded std library is empty");
    }
    if !KENNELS_DIR.entries().is_empty() {
        let _ = KENNELS_DIR.extract(&pkg_path);
    }

    env::set_var("MAID_STD", &std_path);
    env::set_var("MAID_PKG", &pkg_path);
    (std_path, pkg_path)
}

fn main() {
    let _ = ensure_std_available(); // sets env + ensures files exist
    create_package_dir();           // uses MAID_PKG

    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::New { name }), _)      => new_project(Path::new(&name), false),
        (Some(Commands::Init), _)              => new_project(Path::new("."), true),
        (Some(Commands::Install { name }), _)  => add_package(&name),
        (Some(Commands::Remove  { name }), _)  => remove_package(&name),
        (Some(Commands::Update  { name }), _)  => update_package(&name),
        (None, Some(file)) => {
            if let Some(err) = run(&file, None) {
                println!("{err}");
            }
        }
        (None, None) => launch_repl(VERSION),
    }
}
