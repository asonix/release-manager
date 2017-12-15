// This file is part of Release Manager

// Release Manager is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Release Manager is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Release Manager  If not, see <http://www.gnu.org/licenses/>.

#![feature(entry_and_modify)]
#![feature(try_from)]

extern crate toml;
extern crate serde;
extern crate structopt;
extern crate zip;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate structopt_derive;

#[macro_use]
extern crate log;

mod error;
mod target;
mod config;
mod status;
mod commandline;

use std::process::{Command, ExitStatus};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml::Value;

pub use target::{Arch, OS, Target};
pub use error::Error;
pub use config::{Config, ConfigState, ConfigTrait, TargetConfig};
pub use status::{StatusWrapper, Status, VersionStatus, BuildStatus};
pub use commandline::Opt;

pub fn parse_toml<T>(filename: &Path) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(toml::from_str(&contents)?)
}

pub fn table_str<'a>(table: &'a Value, name: &str, missing_err: Error) -> Result<&'a str, Error> {
    table.get(name).ok_or(missing_err).and_then(
        |value| match value {
            &Value::String(ref v) => Ok(v.as_ref()),
            _ => Err(Error::NotString),
        },
    )
}

pub fn publish() -> Result<ExitStatus, Error> {
    Command::new("cargo").arg("publish").status().map_err(
        |e| e.into(),
    )
}
