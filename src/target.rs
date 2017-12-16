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

use std::process::{Command, ExitStatus};
use std::collections::HashMap;
use std::convert::TryFrom;
use super::Error;
use super::StatusWrapper;

#[derive(Clone, Debug)]
pub enum Arch {
    Aarch64,
    Armv7h,
    Armv7hMusl,
    Armh,
    ArmhMusl,
    Amd64,
    Amd64Musl,
    I686,
}

impl<'a> TryFrom<&'a str> for Arch {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Arch, &'a str> {
        match value {
            "aarch64" => Ok(Arch::Aarch64),
            "armv7h" => Ok(Arch::Armv7h),
            "armv7hmusl" => Ok(Arch::Armv7hMusl),
            "armh" => Ok(Arch::Armh),
            "armhmusl" => Ok(Arch::ArmhMusl),
            "amd64" => Ok(Arch::Amd64),
            "amd64musl" => Ok(Arch::Amd64Musl),
            "i686" => Ok(Arch::I686),

            _ => Err(value),
        }
    }
}

#[derive(Clone, Debug)]
pub enum OS {
    Linux,
    Windows,
    Mac,
}

pub struct Target {
    os: OS,
    arch: Arch,
    build_name: Option<String>,
    native_dirs: Vec<String>,
    environment: HashMap<String, String>,
}

impl Target {
    pub fn new(os: OS, arch: Arch, build_name: Option<String>) -> Result<Self, Error> {
        match (&os, &arch) {
            (&OS::Linux, &Arch::Aarch64) |
            (&OS::Linux, &Arch::Armv7h) |
            (&OS::Linux, &Arch::Armv7hMusl) |
            (&OS::Linux, &Arch::Armh) |
            (&OS::Linux, &Arch::ArmhMusl) |
            (&OS::Linux, &Arch::Amd64) |
            (&OS::Linux, &Arch::Amd64Musl) |
            (&OS::Windows, &Arch::Amd64) |
            (&OS::Windows, &Arch::I686) |
            (&OS::Mac, &Arch::Amd64) => {
                Ok(Target {
                    os: os,
                    arch: arch,
                    build_name: build_name,
                    native_dirs: Vec::new(),
                    environment: HashMap::new(),
                })
            }
            _ => Err(Error::InvalidTarget),
        }
    }

    pub fn target_str(&self) -> &str {
        match (&self.os, &self.arch) {
            (&OS::Linux, &Arch::Aarch64) => "aarch64-unknown-linux-gnu",
            (&OS::Linux, &Arch::Armv7h) => "armv7-unknown-linux-gnueabihf",
            (&OS::Linux, &Arch::Armv7hMusl) => "armv7-unknown-linux-musleabihf",
            (&OS::Linux, &Arch::Armh) => "arm-unknown-linux-gnueabihf",
            (&OS::Linux, &Arch::ArmhMusl) => "arm-unknown-linux-musleabihf",
            (&OS::Linux, &Arch::Amd64) => "x86_64-unknown-linux-gnu",
            (&OS::Linux, &Arch::Amd64Musl) => "x86_64-unknown-linux-musl",
            (&OS::Windows, &Arch::Amd64) => "x86_64-pc-windows-gnu",
            (&OS::Windows, &Arch::I686) => "i686-pc-windows-gnu",
            (&OS::Mac, &Arch::Amd64) => "x86_64-apple-darwin",
            _ => {
                debug!("Unkown OS/Arch combination: {:?}, {:?}", self.os, self.arch);
                "unknown"
            }
        }
    }

    pub fn output_str(&self) -> String {
        match self.build_name {
            Some(ref name) => format!("{}-{}", self.target_str(), name),
            None => self.target_str().into(),
        }
    }

    pub fn add_libs(&mut self, libs: &[String]) {
        self.native_dirs.extend_from_slice(libs);
    }

    pub fn add_env(&mut self, env: &HashMap<String, String>) {
        for (key, value) in env {
            self.environment.insert(key.clone(), value.clone());
        }
    }

    pub fn libs(&self) -> String {
        self.native_dirs
            .iter()
            .map(|dir| format!("-L native={}", dir))
            .collect::<Vec<_>>()
            .join(" ")
    }

    // Make env a table so it has key-value pairs
    pub fn compile(&self, version: &str, status: &mut StatusWrapper) -> Result<ExitStatus, Error> {
        debug!("Starting compile for {}", self.output_str());

        Command::new("cargo")
            .args(&["build", "--target", self.target_str(), "--release"])
            .env(
                "RUSTFLAGS",
                &format!("-C target-feature=+crt-static {}", &self.libs()),
            )
            .envs(&self.environment)
            .spawn()
            .map_err(|e| e.into())
            .and_then(|mut child| {
                status.start(self.target_str(), version);
                let _ = status.write();

                child.wait().map_err(|e| e.into())
            })
            .and_then(|exit_status| {
                if exit_status.success() {
                    status.succeed(self.target_str(), version);
                } else {
                    status.fail(self.target_str(), version);
                }
                let _ = status.write();

                Ok(exit_status)
            })
    }
}


impl<'a> TryFrom<&'a str> for OS {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<OS, &'a str> {
        match value {
            "Linux" => Ok(OS::Linux),
            "Windows" => Ok(OS::Windows),
            "Mac" => Ok(OS::Mac),
            _ => Err(value),
        }
    }
}
