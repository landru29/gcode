use clap::{Parser, Subcommand};

use crate::application::drill::drill_gcode;
use crate::errors::cli::CliError;
use crate::models::gcode::GCodePathOptions;
use crate::models::starter::Starter;
use crate::models::geometry::Entity;
use crate::models::finisher::Finisher;
use crate::drawing::file::DxfFile;

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

    /// Drill holes
    Drill {
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

        // Filter on entities (line, point, arc)
        #[arg(long, value_delimiter=',')]
        entities: Vec<String>,

        /// Depth for cutting moves
        #[arg(long, default_value_t = 1.0)]
        deep: f64,
    },

    /// Engrave a path
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

        // Filter on entities (line, point, arc)
        #[arg(long, value_delimiter=',')]
        entities: Vec<String>,

        /// Depth for cutting moves
        #[arg(long, default_value_t = 1.0)]
        deep: f64,

        /// Step for cutting moves
        #[arg(long, default_value_t = 0.5)]
        step: f64,
    },
}

pub fn start_cli() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { security_z } => {
            let starter = Starter::new();
            println!("{}", starter.gcode_path(GCodePathOptions::new(&vec![
                GCodePathOptions::with_security_z(&security_z),
            ])));
            Ok(())
        }
        Commands::Finish { security_z } => {
            let finisher = Finisher::new();
            println!("{}", finisher.gcode_path(GCodePathOptions::new(&vec![
                GCodePathOptions::with_security_z(&security_z),
            ])));
            Ok(())
        }
        Commands::Path { dxf, security_z, feed, layer, entities, deep, step } => {
            let mut dxf_file = DxfFile::new(dxf).map_err(|e| CliError::GenericError(format!("{}", e)))?;
            dxf_file.load().map_err(|e| CliError::GenericError(format!("{}", e)))?;
            let filtered_file = dxf_file.filter_layer(layer, entities).unwrap();
            filtered_file.display();
            Ok(())
        }
        Commands::Drill {dxf, security_z, feed, layer, entities, deep} => {
            let mut dxf_file = DxfFile::new(dxf).map_err(|e| CliError::GenericError(format!("{}", e)))?;;
            dxf_file.load().map_err(|e| CliError::GenericError(format!("{}", e)))?;;
            let filtered_file = dxf_file.filter_layer(layer, entities).unwrap();
            println!("{}", drill_gcode(filtered_file.entities(), security_z, feed, deep));
            Ok(())
        }
    }
}
