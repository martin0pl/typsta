use clap::{Parser, Subcommand};

mod app;
mod utils;
mod commands;

use app::App;
use utils::file_exists_in_home;
use commands::new::command_new;
use commands::list::command_list;
use commands::config::command_config;
use commands::update::command_update;

const DESCRIPTION: &str = "A little tool to help you create new Typst project with a template.";
const CONFIG_FILE_NAME: &str = ".typsta-config.json";

#[derive(Parser)]
#[command(name = "typsta")]
#[command(version)]
#[command(about = DESCRIPTION, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all the template avalaible
    List,
    /// Create a new project with a template
    New {
        /// Name of the project
        project_name: String,
        /// Name of the template
        template_name: String,
    },
    /// Update the current template of your project
    Update,
    /// Show or set the source folder path for templates
    Config {
        /// New path to set. If omitted, displays the current value.
        path: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let app: App = if file_exists_in_home(CONFIG_FILE_NAME) {
        App::load(CONFIG_FILE_NAME)
    } else {
        App::new()
    };

    let template_dir_ok = !app.source_folder.is_empty();

    match cli.command {
        Commands::List => {
            command_list(app, template_dir_ok);
        }
        Commands::New { project_name, template_name } => {
            command_new(app,template_dir_ok,project_name,template_name);
        }
        Commands::Update => {
            command_update(template_dir_ok, app.source_folder);
        }
        Commands::Config { path } => {
            command_config(app, path, CONFIG_FILE_NAME, template_dir_ok);
        },
    }
}
