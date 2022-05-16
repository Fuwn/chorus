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

use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

/// Format an existing Chorus package
#[allow(clippy::unnecessary_wraps)]
pub fn format(path: &Option<PathBuf>) -> anyhow::Result<()> {
  let our_path = path.clone().unwrap_or_else(|| PathBuf::from("."));

  // Only try to format if the target Chorus package directory exists
  if our_path.exists() {
    let mut entries: Vec<DirEntry> = vec![];

    std::env::set_current_dir(&our_path)?;

    // Iterate over all paths in the target Chorus package
    for entry in WalkDir::new(our_path).into_iter().filter_map(Result::ok) {
      // Only track COBOL files to format if the file has a valid COBOL extension
      if entry
        .file_name()
        .to_str()
        .unwrap_or("")
        .rsplit('.')
        .next()
        .map(|extension: &str| {
          #[allow(clippy::blocks_in_if_conditions)]
          {
            let mut valid_extension = false;

            // If any user defined COBOL extensions exist, check against them;
            // otherwise, check against the "cbl" file extension.
            if let Some(valid_extensions) =
              &crate::config::CONFIG.build.extensions
            {
              for an_extension in valid_extensions {
                if valid_extension {
                  break;
                }

                valid_extension = extension.eq_ignore_ascii_case(an_extension);
              }
            } else {
              valid_extension = extension.eq_ignore_ascii_case("cbl");
            }

            valid_extension
          }
        })
        == Some(true)
      {
        entries.push(entry);
      }
    }

    for entry in entries {
      println!("entry: {:?}", entry);
    }
  }

  Ok(())
}
