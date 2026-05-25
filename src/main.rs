mod models;
mod drawing;
mod errors;

use clap::{Parser, Subcommand};

use crate::models::gcode::GCodeOptions;
use crate::models::starter::Starter;
use crate::models::geometry::Entity;

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
        /// Path to the input DXF file
        #[arg(short, long)]
        dxf: String,

        /// Safety height for non-cutting moves
        #[arg(long, default_value_t = 5.0)]
        security_z: f64,

        /// Feed rate for cutting moves
        #[arg(short, long, default_value_t = 100.0)]
        feed: f64,

        /// Pattern for layer filtering, e.g. "0" or "Layer*" or "*"
        #[arg(long, default_value = "*")]
        layer: String,

        /// Depth for cutting moves
        #[arg(long, default_value_t = 1.0)]
        deep: f64,

        /// Step for cutting moves
        #[arg(long, default_value_t = 0.5)]
        step: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { security_z } => {
            let starter = Starter::new();
            println!("{}", starter.gcode_path(GCodeOptions::new(&vec![
                GCodeOptions::with_security_z(&security_z),
                GCodeOptions::with_x(),
                GCodeOptions::with_y(),
                GCodeOptions::with_z(),
            ])));
        }
        Commands::Finish { security_z } => {
            let finisher = models::finisher::Finisher::new(security_z);
            println!("{}", finisher.gcode_path(GCodeOptions::new(&vec![])));
        }
        Commands::Path { dxf, security_z, feed, layer, deep, step } => {
            let mut dxf_file = drawing::file::DxfFile::new(dxf).unwrap();
            dxf_file.load();
            let filtered_file = dxf_file.filter_layer(layer).unwrap();
            filtered_file.display();
        }
    }
}
