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

/// "None if empty string, or with preformat"
///
/// By default, this macro evaluates whether the input `$string` is empty by
/// calling `is_empty`. However, by specifying the type `i32` at the end of the
/// macro invocation, you can tell this macro to evaluate based on `== 0`
/// instead of `is_empty`.
#[macro_export]
macro_rules! niesowp {
  ($preformat:literal, $string:expr) => {{
    if $string.is_empty() {
      "".to_string()
    } else {
      format!("{}{}", $preformat, $string)
    }
  }};
  ($preformat:literal, $number:expr,i32) => {{
    if $number == 0 {
      "".to_string()
    } else {
      format!("{}{}", $preformat, $number)
    }
  }};
}

#[macro_export]
macro_rules! compiler_get_or {
  ($compiler:ident, $key:ident, $default:literal) => {
    $crate::config::CONFIG.build.$compiler.as_ref().map_or_else(
      || $default.into(),
      |compiler| {
        compiler
          .$key
          .as_ref()
          .map_or_else(|| $default.into(), Clone::clone)
      },
    )
  };
}

/// A helper to assist with the sanity checks
#[macro_export]
macro_rules! entry_exists {
  ($entries:ident, $path:ident, $entry:literal, $context:literal) => {
    let entry = $entry.replace("\\", "/");

    if !$entries.contains(&PathBuf::from_slash(
      format!("{}/{}", $path.as_path().display(), entry).to_string(),
    )) {
      return Err(anyhow::anyhow!(
        "{} \"{}\" does not exist",
        $context,
        format!("{}/{}", $path.as_path().display(), entry)
      ));
    }
  };
}
