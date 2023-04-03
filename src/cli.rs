use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::Action;

/// KSP1 ConfigNode parser and block removal tool
#[derive(Parser, PartialEq, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Commands {
    /// remove blocks from the file(s)
    Remove {
        #[command(subcommand)]
        node: Node,
    },
    /// read the file(s) and replace with a clean version
    Clean {
        /// file(s) to apply the command to
        files: Vec<PathBuf>,
    },
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Node {
    /// Remove all instances of a Part
    Part {
        /// The part name
        name: String,
        /// file(s) to apply the command to
        files: Vec<PathBuf>,
    },
    /// Remove all instances of a Module
    Module {
        /// The module name
        name: String,
        /// file(s) to apply the command to
        files: Vec<PathBuf>,
    },
    /// Remove all instances of a Resource
    Resource {
        /// The resource name
        name: String,
        /// file(s) to apply the command to
        files: Vec<PathBuf>,
    },
}

pub fn parse_action() -> (Action, Vec<PathBuf>) {
    let args = Cli::parse();

    match args.command {
        Commands::Remove { node } => {
            match node {
                Node::Part { name, files } => {
                    ( Action::Remove("PART".to_owned(), name), files)
                },
                Node::Resource { name, files } => {
                    ( Action::Remove("RESOURCE".to_owned(), name), files)
                },
                Node::Module { name, files } => {
                    ( Action::Remove("MODULE".to_owned(), name), files)
                },
            }
        }
        Commands::Clean { files } => {
            ( Action::Clean, files)
        }
    }
}
