use clap::{Parser, Subcommand};

mod app;
mod utils;

use app::App;
use utils::{expand_tilde, file_exists_in_home, templates_names};

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

    let mut app: App = if file_exists_in_home(CONFIG_FILE_NAME) {
        App::load(CONFIG_FILE_NAME)
    } else {
        App::new()
    };

    let template_dir_ok = !app.source_folder.is_empty();

    match cli.command {
        Commands::List => {
            if template_dir_ok {
                let source_path = expand_tilde(&app.source_folder);

                let dir_names: Vec<String> = templates_names(source_path);

                for name in &dir_names {
                    println!("{name}");
                }
            } else {
                println!("Please configurate your template directory with : typsta config <path>");
            }
        }
        Commands::New { project_name, template_name } => {
            if template_dir_ok {
                // TODO
            } else {
                println!("Please configurate your template directory with : typsta config <path>");
            }
        }
        Commands::Update => {
            if template_dir_ok {
                // TODO
            } else {
                println!("Please configurate your template directory with : typsta config <path>");
            }
        }
        Commands::Config { path } => match path {
            Some(new_path) => {
                app.source_folder = new_path;
                app.save(CONFIG_FILE_NAME);
                println!("Source folder set to : {}", app.source_folder);
            }
            None => {
                if template_dir_ok {
                    println!("Current source folder : {}", app.source_folder);
                } else {
                    println!("Please configurate your template directory with : typsta config <path>");
                }
            }
        },
    }
}
