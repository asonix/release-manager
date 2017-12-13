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
            match os.as_ref() {
                "Linux" => {
                    build_os(&mut targets, OS::Linux, value);
                }
                "Windows" => {
                    build_os(&mut targets, OS::Windows, value);
                }
                "Mac" => {
                    build_os(&mut targets, OS::Mac, value);
                }
                _ => {
                    debug!("{} not a valid Operating System", os);
                }
            }
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
        match arch.as_ref() {
            "aarch64" => {
                add_target(targets, os.clone(), Arch::Aarch64, tc);
            }
            "armv7h" => {
                add_target(targets, os.clone(), Arch::Armv7h, tc);
            }
            "armv7hmusl" => {
                add_target(targets, os.clone(), Arch::Armv7hMusl, tc);
            }
            "armh" => {
                add_target(targets, os.clone(), Arch::Armh, tc);
            }
            "armhmusl" => {
                add_target(targets, os.clone(), Arch::ArmhMusl, tc);
            }
            "amd64" => {
                add_target(targets, os.clone(), Arch::Amd64, tc);
            }
            "amd64musl" => {
                add_target(targets, os.clone(), Arch::Amd64Musl, tc);
            }
            "i686" => {
                add_target(targets, os.clone(), Arch::I686, tc);
            }
            _ => {
                debug!("{} not a valid architecture", arch);
            }
        }
    }
}
