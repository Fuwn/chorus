// This file is part of Chorus <https://github.com/Fuwn/chorus>.
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Chorus {
  #[clap(subcommand)]
  command: Commands,

  /// Enable debug output
  #[clap(short, long, parse(from_occurrences))]
  debug: usize,
}

#[derive(Subcommand)]
enum Commands {
  /// Create a new Chorus package in an existing directory
  Init {
    /// The location to create the new Chorus package at
    path: Option<std::path::PathBuf>,
  },
  /// Create a new Chorus package in a new directory
  New {
    /// The location to create the new Chorus project at
    path: std::path::PathBuf,
  },
  /// Format all COBOL files within a Chorus package
  Fmt {
    /// The location of the target Chorus package to format
    #[clap(short, long)]
    path: Option<std::path::PathBuf>,
  },
  /// Compile the Chorus package within the current directory
  Build {
    /// The location of the target Chorus package to build
    #[clap(short, long)]
    path:   Option<std::path::PathBuf>,
    #[clap(short, long)]
    output: Option<std::path::PathBuf>,
  },
}

/// Setup and handle command-line interactions
pub fn evaluate() {
  // Set up the CLI
  let chorus = Chorus::parse();

  // Handle CLI command
  match &chorus.command {
    Commands::Init {
      path,
    } => {
      if let Err(why) = commands::init_new(path, true) {
        println!("error: could not initialise package: {}", why);
        std::process::exit(1);
      };
    }
    Commands::New {
      path,
    } => {
      if let Err(why) = commands::init_new(&Some(path.clone()), false) {
        println!("error: could not create new package: {}", why);
        std::process::exit(1);
      };
    }
    Commands::Fmt {
      path,
    } => {
      if let Err(why) = commands::format(path) {
        println!("error: could not format package: {}", why);
        std::process::exit(1);
      };
    }
    Commands::Build {
      path,
      output,
    } => {
      if let Err(why) = commands::build(path, output) {
        println!("error: could not build package: {}", why);
        std::process::exit(1);
      };
    }
  }
}
