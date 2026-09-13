use clap::{Parser, Subcommand};

const DESCRIPTION: &str = "A little tool to help you create new Typst project with a template.";

#[derive(Parser)]
#[command(name = "croute")]
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
        /// Title of the template
        title: String,
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

    match &cli.command {
        Commands::List => {
            println!("List");
        },
        Commands::New {title} => {
            println!("List");
        },
        Commands::Update => {
            println!("Update");
        },
        Commands::Config { path } => match path {
            Some(new_path) => {
                println!("Config set");
            }
            None => {
                println!("Config show");
            }
        },

    }
}
