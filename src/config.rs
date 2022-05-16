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

use serde_derive::Deserialize;

lazy_static::lazy_static! {
  pub static ref CONFIG: Config = {
    let config = Config::new();

    if let Err(why) = config {
      println!("error: could not read config: {}", why);
      std::process::exit(1);
    }

    config.unwrap()
  };
}

#[derive(Deserialize)]
pub struct Package {
  pub name: String,
}

#[derive(Deserialize)]
pub struct Build {
  pub compiler:   String,
  pub extensions: Option<Vec<String>>,
  pub mode:       Option<String>,
  pub gnucobol:   Option<crate::compilers::gnucobol::GNUCobol>,
  pub r#type:     Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
  pub package: Package,
  pub build:   Build,
}
impl Config {
  /// Initialise a new `Config`
  pub fn new() -> Result<Self, config::ConfigError> {
    config::Config::builder()
      .add_source(config::File::with_name("package.toml"))
      .build()?
      .try_deserialize()
  }
}
