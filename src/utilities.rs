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

use anyhow::anyhow;
use unicode_xid::UnicodeXID;

/// Check if a string is a valid Unicode XID identifier
pub fn is_unicode_xid(input: &str) -> anyhow::Result<()> {
  let mut input_scroller = input.chars();

  if !UnicodeXID::is_xid_start(input_scroller.next().unwrap()) {
    return Err(anyhow!(
      "identifier \"{}\" is not valid unicode xid identifier",
      input
    ));
  }

  for character in input_scroller {
    if !UnicodeXID::is_xid_continue(character) {
      return Err(anyhow!(
        "identifier \"{}\" is not valid unicode xid identifier",
        input
      ));
    }
  }

  Ok(())
}
