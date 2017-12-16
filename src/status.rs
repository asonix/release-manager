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
use std::path::Path;
use std::fs::File;
use std::io::Write;

use super::parse_toml;
use super::Error;

use toml;

type Version = String;
type BuildName = String;

pub type Status = HashMap<Version, VersionStatus>;

pub struct StatusWrapper<'a> {
    pub filepath: &'a Path,
    pub status: Status,
}

impl<'a> StatusWrapper<'a> {
    pub fn new(filepath: &'a Path) -> Self {
        StatusWrapper {
            status: Status::new(),
            filepath: filepath,
        }
    }

    pub fn read(&mut self) -> Result<(), Error> {
        self.status = parse_toml(self.filepath)?;

        Ok(())
    }

    pub fn write(&self) -> Result<(), Error> {
        let mut f = File::create(self.filepath)?;
        write!(f, "{}", toml::to_string(&self.status)?)?;

        Ok(())
    }

    pub fn published(&mut self, version: &str) {
        self.status.entry(version.into()).and_modify(
            |version_status| {
                version_status.published = true;
            },
        );
    }

    pub fn clear_missing_targets(&mut self, version: &str, target_strings: &[String]) {
        let mut version_info = self.status.get_mut(version);
        if let Some(ref mut version_info) = version_info {
            version_info.build_names.retain(|k, _| {
                let contains = target_strings.contains(&k);
                if !contains {
                    debug!(
                        "{} not in supplied release config, removing from status file",
                        k
                    )
                }
                contains
            });
        }
    }

    pub fn needs_compile(&self, build_name: &str, version: &str) -> bool {
        let build_names = self.status.get(version);
        let vs = if let Some(vs) = build_names {
            vs
        } else {
            return true;
        };

        let build_status = vs.build_names.get(build_name);
        let build_status = if let Some(build_status) = build_status {
            build_status
        } else {
            return true;
        };

        match *build_status {
            BuildStatus::Success => false,
            _ => true,
        }
    }

    fn set_status(&mut self, build_name: &str, version: &str, status: BuildStatus) {
        let version_status = self.status.entry(version.into()).or_insert(
            VersionStatus::default(),
        );
        version_status.build_names.insert(build_name.into(), status);
    }

    pub fn start(&mut self, build_name: &str, version: &str) {
        self.set_status(build_name, version, BuildStatus::Started);
    }

    pub fn succeed(&mut self, build_name: &str, version: &str) {
        self.set_status(build_name, version, BuildStatus::Success);
    }

    pub fn fail(&mut self, build_name: &str, version: &str) {
        self.set_status(build_name, version, BuildStatus::Failed);
    }

    pub fn reset_all(&mut self, version: &str) {
        let version_status = self.status.entry(version.into()).or_insert(
            VersionStatus::default(),
        );

        for value in version_status.build_names.values_mut() {
            *value = BuildStatus::Waiting;
        }
    }

    pub fn all_clear(&self, version: &str) -> bool {
        let version_status = self.status.get(version);

        if let Some(ref version_status) = version_status {
            for build_status in version_status.build_names.values() {
                if build_status != &BuildStatus::Success {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn publish(&mut self, version: &str) {
        let version_status = self.status.entry(version.into()).or_insert(
            VersionStatus::default(),
        );
        version_status.published = true;
    }

    pub fn is_published(&self, version: &str) -> bool {
        let version_status = self.status.get(version);

        if let Some(version_status) = version_status {
            version_status.published
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VersionStatus {
    published: bool,
    build_names: HashMap<BuildName, BuildStatus>,
}

impl Default for VersionStatus {
    fn default() -> Self {
        VersionStatus {
            published: false,
            build_names: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum BuildStatus {
    Waiting,
    Started,
    Success,
    Failed,
}
