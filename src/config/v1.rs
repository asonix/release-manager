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

use std::collections::HashMap;
use std::convert::TryFrom;

use super::{ConfigTrait, TargetConfig, Target, OS};
use super::build_os;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub release_path: String,
    pub included_files: Vec<String>,
    pub config: HashMap<String, HashMap<String, TargetConfig>>,
}

impl Config {
    pub fn included_files(&self) -> &[String] {
        self.included_files.as_ref()
    }
}

impl ConfigTrait for Config {
    type Previous = super::v0::Config;

    fn targets(&self) -> Vec<Target> {
        let mut targets = Vec::new();

        for (os, value) in self.config.iter() {
            let opsys = OS::try_from(os.as_ref());
            if opsys.is_err() {
                debug!("{}, is not a valid Operating System!", os);
            } else {
                build_os(&mut targets, opsys.unwrap(), value);
            }
        }

        targets
    }
}

impl From<super::v0::Config> for Config {
    fn from(c: super::v0::Config) -> Self {
        let mut v = Vec::new();
        v.push(c.readme);
        v.push(c.license);

        Config {
            release_path: c.release_path,
            included_files: v,
            config: c.config,
        }
    }
}
