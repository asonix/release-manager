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

use super::{ConfigTrait, Target, Arch, OS};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub release_path: String,
    pub included_files: Vec<String>,
    pub config: HashMap<String, HashMap<String, Vec<TargetConfig>>>,
}

impl Config {
    pub fn included_files(&self) -> &[String] {
        self.included_files.as_ref()
    }
}

impl ConfigTrait for Config {
    type Previous = super::v1::Config;

    fn targets(&self) -> Vec<Target> {
        let mut targets = Vec::new();

        for (os_str, value) in self.config.iter() {
            if let Ok(os) = OS::try_from(os_str.as_ref()) {
                build_os(&mut targets, os, value);
            } else {
                debug!("{}, is not a valid Operating System!", os_str);
            }
        }

        targets
    }
}

impl From<super::v1::Config> for Config {
    fn from(c: super::v1::Config) -> Self {
        let mut config: HashMap<String, HashMap<String, Vec<TargetConfig>>> = HashMap::new();

        for (os, arch_hash) in c.config {
            for (arch, target) in arch_hash {
                let os_entry = config.entry(os.clone().into()).or_insert(HashMap::new());
                let arch_entry = os_entry.entry(arch.into()).or_insert(Vec::new());
                arch_entry.push(target.into());
            }
        }

        Config {
            release_path: c.release_path,
            included_files: c.included_files,
            config: config,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetConfig {
    build_name: Option<String>,
    libs: Vec<String>,
    env: HashMap<String, String>,
}

impl From<super::TargetConfig> for TargetConfig {
    fn from(tc: super::TargetConfig) -> Self {
        TargetConfig {
            build_name: None,
            libs: tc.libs,
            env: tc.env,
        }
    }
}

fn add_target(targets: &mut Vec<Target>, os: OS, arch: Arch, tc: &TargetConfig) {
    if let Ok(mut target) = Target::new(os.clone(), arch, tc.build_name.clone()) {
        target.add_libs(&tc.libs);
        target.add_env(&tc.env);
        targets.push(target);
    }
}

fn build_os(targets: &mut Vec<Target>, os: OS, value: &HashMap<String, Vec<TargetConfig>>) {
    for (arch, tcs) in value.iter() {
        if let Ok(arc) = Arch::try_from(arch.as_ref()) {
            for target in tcs {
                add_target(targets, os.clone(), arc.clone(), target);
            }
        } else {
            debug!("{} is not a valid architecture!", arch);
        }
    }
}
