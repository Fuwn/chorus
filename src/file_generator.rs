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

use std::path::Path;

use anyhow::Result;

use crate::utilities::is_unicode_xid;

/// Keep unique paths, replace local paths (".") with "default"
fn local_or_default(path: &Path) -> String {
  if path.display().to_string() == "." {
    "default".to_string()
  } else {
    path.display().to_string()
  }
}

/// Generate a default main file
pub fn main_file(name: &Path) -> Result<String> {
  is_unicode_xid(&name.display().to_string())?;

  Ok(format!(
    r#"IDENTIFICATION DIVISION.
PROGRAM-ID. {}.

PROCEDURE DIVISION.
DISPLAY "Hello, World!".
STOP RUN.
"#,
    local_or_default(name).to_ascii_uppercase(),
  ))
}

/// Generate a Chorus package manifest
pub fn package_manifest(name: &Path) -> Result<String> {
  is_unicode_xid(&name.display().to_string())?;

  Ok(format!(
    r#"[package]
name = "{}"

[build]
compiler = "gnucobol"
extensions = ["cbl", "cob"]
mode = "incremental"
type = "executable"

[build.gnucobol]
format = "free"
options = ["-Wall"]
std = "default"
optimization = "O3"
"#,
    local_or_default(name),
  ))
}
