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

use super::{Arch, OS, Target};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub release_path: String,
    pub license: String,
    pub readme: String,
    pub config: HashMap<String, HashMap<String, TargetConfig>>,
}

impl Config {
    pub fn targets(&self) -> Vec<Target> {
        let mut targets = Vec::new();

        for (os, value) in self.config.iter() {
            let opsys = OS::try_from(os.as_ref());
            build_os(&mut targets, opsys.expect(&format!("{}, is not a valid Operating System!",os)), value);
        }

        targets
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetConfig {
    libs: Vec<String>,
    env: HashMap<String, String>,
}

fn add_target(targets: &mut Vec<Target>, os: OS, arch: Arch, tc: &TargetConfig) {
    if let Ok(mut target) = Target::new(os.clone(), arch) {
        target.add_libs(&tc.libs);
        target.add_env(&tc.env);
        targets.push(target);
    }
}

fn build_os(targets: &mut Vec<Target>, os: OS, value: &HashMap<String, TargetConfig>) {
    for (arch, tc) in value.iter() {
        let arc = Arch::try_from(arch.as_ref());
        add_target(targets, os.clone(), arc.expect(&format!("{} is not a valid architecture!", arch)), tc);
    }
}
