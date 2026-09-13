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

    }
}
