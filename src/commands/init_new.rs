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

use std::{io::Write, path::PathBuf};

use anyhow::Context;

use crate::file_generator;

/// A wrapper of `std::fs::File::create` and `write!` which creates, writes, and
/// error checks a file given a base path and file name.
macro_rules! write_new_file {
  ($path:ident, $filepath:expr, $content:expr) => {{
    // Save the file path for later use
    let file =
      format!("{}{}", format!("{}/", $path.as_path().display()), $filepath);

    // Create and write to the new file with error handling
    write!(
      &mut std::fs::File::create(&file)
        .with_context(|| format!("could not create file: \"{}\"", file))?,
      "{}",
      $content,
    )
    .with_context(|| format!("could not write to file: \"{}\"", file))?;
  }};
}

#[macro_export]
/// A wrapper of `std::fs::create_dir_all` which creates and error checks a
/// directory given a base path and directory name.
macro_rules! create_directory {
  ($path:ident, $directory_path:expr) => {{
    // Save the directory path for later use
    let directory = format!(
      "{}{}",
      format!("{}/", $path.as_path().display()),
      $directory_path
    );

    // Create the new directory with error handling
    std::fs::create_dir(&directory).with_context(|| {
      format!("could not create directory: \"{}\"", directory)
    })?;
  }};
  ($directory_path:expr) => {{
    // Create the new directory with error handling
    std::fs::create_dir(&$directory_path).with_context(|| {
      format!("could not create directory: \"{}\"", $directory_path)
    })?;
  }};
}

/// Initialise or create a new Chorus package
pub fn init_new(path: &Option<PathBuf>, init: bool) -> anyhow::Result<()> {
  let our_path = path.clone().unwrap_or_else(|| PathBuf::from("."));

  crate::utilities::is_unicode_xid(&our_path.display().to_string())?;

  // Make sure that we are not initialing a new Chorus project in a non-empty
  // directory.
  if init && our_path.exists() && our_path.read_dir()?.next().is_some() {
    return Err(anyhow::anyhow!(
      "target directory \"{}\" is not empty",
      our_path.as_path().display()
    ));
  }

  // Set up the new Chorus package with the default file structure of a Chorus
  // package.
  create_directory!(our_path, "");
  create_directory!(our_path, "src");
  write_new_file!(
    our_path,
    "src/main.cbl",
    file_generator::main_file(&our_path)?
  );
  write_new_file!(
    our_path,
    "package.toml",
    file_generator::package_manifest(&our_path)?
  );

  Ok(())
}
