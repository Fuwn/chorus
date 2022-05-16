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

use std::error::Error;

use shellfn::shell;

#[derive(serde_derive::Deserialize)]
pub struct GNUCobol {
  pub format:       Option<String>,
  pub options:      Option<Vec<String>>,
  pub std:          Option<String>,
  pub optimization: Option<String>,
}

/// Compile an executable Chorus package in a single command
#[shell]
pub fn compile_executable_single(
  compiler: &str,
  flags: &str,
  files: &str,
  output: &str,
) -> Result<String, Box<dyn Error>> {
  r#"$COMPILER -x $FLAGS $FILES -o $OUTPUT"#
}

/// Compile an ordinary file into an object
#[shell]
pub fn compile_object(
  compiler: &str,
  flags: &str,
  file: &str,
  output: &str,
) -> Result<String, Box<dyn Error>> {
  r#"$COMPILER -c $FLAGS $FILE -o $OUTPUT"#
}

/// Compile an executable file into an object
#[shell]
pub fn compile_executable_object(
  compiler: &str,
  flags: &str,
  file: &str,
  output: &str,
) -> Result<String, Box<dyn Error>> {
  r#"$COMPILER -c -x $FLAGS $FILE -o $OUTPUT"#
}

/// Compile an ordinary file into a module
#[shell]
pub fn compile_module(
  compiler: &str,
  flags: &str,
  file: &str,
  output: &str,
) -> Result<String, Box<dyn Error>> {
  r#"$COMPILER -m $FLAGS $FILE -o $OUTPUT"#
}
