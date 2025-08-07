use clap::{Parser, Subcommand};
use std::{env, path::PathBuf};

use maid_lang::{create_package_dir, new_project, add_package, remove_package, update_package, run, launch_repl};

// === NEW: embed stdlib ===
use include_dir::{include_dir, Dir};
static STD_DIR: Dir<'_>     = include_dir!("$CARGO_MANIFEST_DIR/library");
// If you want to ship a starter kennels file/folder, embed it too (optional)
static KENNELS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/kennels");

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

// Self-install stdlib to a user data dir and set envs so lib.maid can fetch it.
fn ensure_std_available() -> (PathBuf, PathBuf) {
    // Honor user overrides if they already set env vars.
    if let (Ok(std), Ok(pkg)) = (env::var("MAID_STD"), env::var("MAID_PKG")) {
        return (PathBuf::from(std), PathBuf::from(pkg));
    }

    // e.g. Windows: %LOCALAPPDATA%\maid, Linux: ~/.local/share/maid, macOS: ~/Library/Application Support/maid
    let base = dirs::data_local_dir()
        .or_else(dirs::home_dir)
        .unwrap()
        .join("maid");

    let std_path = base.join("library");
    let pkg_path = base; // your code expects kennels.maid under MAID_PKG root

    // Extract embedded std if missing (first run)
    if !std_path.exists() {
        STD_DIR.extract(&std_path).expect("extract std library");
    }
    // Optional: if you ship any starter files in /kennels, extract them
    let kennels_target = pkg_path.join("kennels");
    if KENNELS_DIR.entries().len() > 0 && !kennels_target.exists() {
        KENNELS_DIR.extract(&kennels_target).ok();
    }

    env::set_var("MAID_STD", &std_path);
    env::set_var("MAID_PKG", &pkg_path);
    (std_path, pkg_path)
}

fn main() {
    // Make sure std is present and envs are set BEFORE calling any maid_lang APIs.
    let _ = ensure_std_available();

    // This likely writes/ensures kennels.maid, etc. — now it will use MAID_PKG we set above.
    create_package_dir();

    let cli = Cli::parse();

    match (cli.command, cli.file) {
        (Some(Commands::New { name }), _) => new_project(std::path::Path::new(&name), false),
        (Some(Commands::Init),          _) => new_project(std::path::Path::new("."), true),
        (Some(Commands::Install { name }), _) => add_package(&name),
        (Some(Commands::Remove  { name }), _) => remove_package(&name),
        (Some(Commands::Update  { name }), _) => update_package(&name),
        (None, Some(file)) => {
            if let Some(err) = run(&file, None) {
                println!("{err}");
            }
        }
        (None, None) => launch_repl(VERSION),
    }
}
