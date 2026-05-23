mod geometry;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gcode")]
#[command(about = "A CLI tool for G-code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process a G-code file
    Process {
        /// Input file path
        #[arg(value_name = "FILE")]
        file: String,

        /// Output file path (optional)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Display G-code file information
    Info {
        /// File to analyze
        #[arg(value_name = "FILE")]
        file: String,
    },
    /// Validate G-code syntax
    Validate {
        /// File to validate
        #[arg(value_name = "FILE")]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Process { file, output } => {
            println!("Processing file: {}", file);
            if let Some(out) = output {
                println!("Output will be saved to: {}", out);
            }
        }
        Commands::Info { file } => {
            println!("Getting information for: {}", file);
        }
        Commands::Validate { file } => {
            println!("Validating: {}", file);
        }
    }
}
