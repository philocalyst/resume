use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use std::fs;
use std::path::PathBuf;

mod resumes;
mod types;
use crate::resumes::Available;
use crate::types::Resume;

#[derive(Parser)]
#[command(name = "resume-bakery")]
#[command(about = "A CLI to bake JSON resumes into PDFs using Typst", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new resume.toml file
    Init {
        #[arg(short, long, default_value = "resume.toml")]
        output: PathBuf,
    },
    /// Validate the resume.toml against the internal schema
    Validate {
        #[arg(short, long, default_value = "resume.toml")]
        input: PathBuf,
    },
    /// Export the resume to a specific format
    Export {
        /// Path to the resume.toml file
        #[arg(short, long, default_value = "resume.toml")]
        input: PathBuf,

        /// Path for the output file
        #[arg(short, long, default_value = "output.pdf")]
        output: PathBuf,

        /// The typst template to use
        #[arg(short, long)]
        template: Available,

        /// Output format
        #[arg(short, long, value_enum, default_value_t = Format::Pdf)]
        format: Format,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Pdf,
}

macro_rules! bake_doc {
    ($variant:expr, $inputs:expr, { $($arm:ident => $file:literal),* $(,)? }) => {
        match $variant {
            $(
                Available::$arm => {
                    typst_bake::document!($file).with_inputs($inputs)
                }
            )*
        }
    };
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { output } => {
            let default_resume = toml::to_string_pretty(&Resume::default())?;
            fs::write(&output, default_resume)
                .with_context(|| format!("Failed to create {:?}", output))?;
            println!("Initialized new resume at {:?}", output);
        }
        Commands::Validate { input } => {
            let data = fs::read_to_string(&input)
                .with_context(|| format!("Could not read {:?}", input))?;
            let _: Resume = toml::from_str(&data)
                .with_context(|| "Validation failed: JSON does not match Resume schema")?;
            println!("{:?}: Valid.", input);
        }
        Commands::Export {
            input,
            output,
            template,
            format: _,
        } => {
            // Read and parse
            let data = fs::read_to_string(&input)
                .with_context(|| format!("Could not read {:?}", input))?;
            let resume: Resume =
                toml::from_str(&data).with_context(|| "Failed to parse resume JSON")?;

            // Bake with Typst
            let source_document = bake_doc!(template, resume, {
                Basic => "template.typ",
                // Modern => "modern.typ",
            });

            println!("Baking resume using template: {}...", template);
            let doc = source_document
                .to_pdf()
                .map_err(|e| anyhow::anyhow!("Typst rendering error: {:?}", e))?;
            fs::write(&output, doc)
                .with_context(|| format!("Failed to write PDF to {:?}", output))?;
            println!("Success! Resume exported to {:?}", output);
        }
    }

    Ok(())
}
