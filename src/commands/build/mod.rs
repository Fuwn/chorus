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

mod macros;

use std::path::PathBuf;

use anyhow::{anyhow, Context};
use path_slash::PathBufExt;
use walkdir::{DirEntry, WalkDir};

use crate::{
  compiler_get_or,
  compilers::gnucobol,
  config::CONFIG,
  create_directory,
  entry_exists,
  niesowp,
};

/// Build an existing Chorus package
pub fn build(
  path: &Option<PathBuf>,
  output: &Option<PathBuf>,
) -> anyhow::Result<()> {
  let our_path: PathBuf = path.clone().unwrap_or_else(|| PathBuf::from("."));
  let entries = WalkDir::new(&our_path)
    .into_iter()
    .filter_map(Result::ok)
    .into_iter()
    .map(|e: DirEntry| e.path().to_owned())
    .collect::<Vec<PathBuf>>();

  // Sanity checks to make sure the target directory is a Chorus package
  entry_exists!(entries, our_path, "src", "directory");
  entry_exists!(entries, our_path, "src/main.cbl", "file");
  entry_exists!(entries, our_path, "package.toml", "directory");

  std::env::set_current_dir(&our_path)?;

  let our_output = output.clone().unwrap_or_else(|| {
    PathBuf::from(format!("out/{}", CONFIG.package.name).as_str())
  });

  if !PathBuf::from("out").exists() {
    create_directory!("out");
  }

  if !our_output
    .parent()
    .ok_or_else(|| {
      anyhow!(
        "parent directory of output directory \"{}\" does not exist",
        our_output.display()
      )
    })?
    .exists()
  {
    return Err(anyhow!(
      "output directory \"{}\" does not exist",
      our_output.display()
    ));
  }

  dispatch_build(
    &compiler_get_or!(gnucobol, format, "free"),
    &niesowp!("-", compiler_get_or!(gnucobol, optimization, "")),
    &niesowp!("-std=", compiler_get_or!(gnucobol, std, "")),
    &our_output,
  )?;

  Ok(())
}

fn dispatch_build(
  format: &str,
  optimization: &str,
  standard: &str,
  our_output: &std::path::Path,
) -> anyhow::Result<()> {
  if let Some(mode) = &CONFIG.build.mode {
    if mode.eq_ignore_ascii_case("incremental") {
      if let Err(why) = gnucobol::compile_executable_object(
        "cobc",
        &format!("-{} {} {}", format, optimization, standard),
        "src/main.cbl",
        &format!("{}.o", &our_output.display().to_string()),
      ) {
        return Err(anyhow!(why.to_string()));
      }
      if let Err(why) = gnucobol::compile_executable_single(
        "cobc",
        &format!("-{} {} {}", format, optimization, standard),
        &format!("{}.o", &our_output.display().to_string()),
        &our_output.display().to_string(),
      ) {
        return Err(anyhow!(why.to_string()));
      }
    } else if let Err(why) = gnucobol::compile_executable_single(
      "cobc",
      &format!("-{} {} {}", format, optimization, standard),
      "src/main.cbl",
      &our_output.display().to_string(),
    ) {
      return Err(anyhow!(why.to_string()));
    }
  } else if let Err(why) = gnucobol::compile_executable_single(
    "cobc",
    &format!("-{} {} {}", format, optimization, standard),
    "src/main.cbl",
    &our_output.display().to_string(),
  ) {
    return Err(anyhow!(why.to_string()));
  }

  Ok(())
}
