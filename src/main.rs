mod geometry;
mod drawing;

use clap::{Parser, Subcommand};

use crate::geometry::starter::Starter;
use crate::geometry::geometry::Entity;

#[derive(Parser)]
#[command(name = "gcode")]
#[command(about = "A CLI tool for G-code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a G-code script
    Start {
        #[arg(long, default_value_t = 5.0)]
        security_z: f64,
    },

    /// Finish a G-code script
    Finish {
        #[arg(long, default_value_t = 5.0)]
        security_z: f64,
    },

    Path {
        #[arg(short, long)]
        dxf: String,

        #[arg(long, default_value_t = 5.0)]
        security_z: f64,

        #[arg(short, long, default_value_t = 100.0)]
        feed: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { security_z } => {
            let starter = Starter::new(security_z);
            println!("{}", starter.to_gcode(0.0, false));
        }
        Commands::Finish { security_z } => {
            let finisher = geometry::finisher::Finisher::new(security_z);
            println!("{}", finisher.to_gcode(0.0, false));
        }
        Commands::Path { dxf, security_z, feed } => {
            let dxf_file = drawing::file::DxfFile::new(dxf).unwrap();
            dxf_file.display();
        }
    }
}
