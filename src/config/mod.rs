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

use std::convert::TryFrom;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use toml;

use super::{Arch, OS, Target, Error};
use super::parse_toml;

mod v0;
mod v1;
mod v2;

pub use self::v2::Config;

#[derive(Debug, PartialEq)]
pub enum ConfigState {
    Current,
    Upgraded,
}

pub trait ConfigTrait: DeserializeOwned + Serialize + Sized {
    type Previous: ConfigTrait + Into<Self>;

    fn new(filename: &Path) -> Result<(Self, ConfigState), Error> {
        parse_toml(filename)
            .map(|c| (c, ConfigState::Current))
            .or_else(|e| {
                Self::Previous::new(filename)
                    .map(|(c, _)| {
                        debug!("Upgrading from older config version");
                        (c.into(), ConfigState::Upgraded)
                    })
                    .map_err(|_| e)
            })
    }

    fn targets(&self) -> Vec<Target>;

    fn save(&self, path: &Path) -> Result<(), Error> {
        let mut f = File::create(path)?;
        write!(f, "{}", toml::to_string(&self)?)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmptyConfig;

impl ConfigTrait for EmptyConfig {
    type Previous = EmptyConfig;

    fn new(_: &Path) -> Result<(Self, ConfigState), Error> {
        Err(Error::Config)
    }

    fn targets(&self) -> Vec<Target> {
        Vec::new()
    }

    fn save(&self, _: &Path) -> Result<(), Error> {
        Err(Error::Config)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetConfig {
    libs: Vec<String>,
    env: HashMap<String, String>,
}

fn add_target(targets: &mut Vec<Target>, os: OS, arch: Arch, tc: &TargetConfig) {
    if let Ok(mut target) = Target::new(os.clone(), arch, None) {
        target.add_libs(&tc.libs);
        target.add_env(&tc.env);
        targets.push(target);
    }
}

fn build_os(targets: &mut Vec<Target>, os: OS, value: &HashMap<String, TargetConfig>) {
    for (arch, tc) in value.iter() {
        if let Ok(arc) = Arch::try_from(arch.as_ref()) {
            add_target(targets, os.clone(), arc, tc);
        } else {
            debug!("{} is not a valid architecture!", arch);
        }
    }
}
