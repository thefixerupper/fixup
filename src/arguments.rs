// SPDX-License-Identifier: MIT
// Copyright 2024 Martin Furman

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Command line arguments.
#[derive(Debug, Parser)]
#[command(name = "fixup")]
#[command(about = "FurOS package manager")]
#[command(version = "0.1.0")]
pub struct Arguments {
    /// What action should be performed with packages?
    #[arg(value_enum)]
    #[arg(help = "The action that should be performed")]
    pub action: Action,

    #[arg(short, long)]
    #[arg(help = "Force reinstalling the package even if already installed")]
    pub force: bool,

    /// What packages to perform the action with?
    #[arg(help = "The name(s) of the package(s) to be installed")]
    pub package: Vec<PathBuf>,
}

impl Arguments {
    /// Get the command line arguments used to launch this instance.
    pub fn get() -> Self {
        Arguments::parse()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum Action {
    #[value(help = "Install one or more packages into the system")]
    #[value(alias = "i")]
    Install,

    #[value(help = "Remove one or more packages from the system")]
    #[value(alias = "r")]
    Remove,

    #[value(help = "Get information about one or more packages")]
    #[value(alias = "d")]
    Describe,
}

