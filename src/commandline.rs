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

#[derive(StructOpt, Debug)]
#[structopt(name = "release-manager",
            about = "A utility for creating release binaries for multiple platforms")]
pub struct Opt {
    #[structopt(short = "f", long = "force", help = "Force recompiling of succeeded builds")]
    force_compile: bool,
    #[structopt(long = "skip-dependencies", help = "Don't compile dependencies unless needed")]
    skip_dependencies: bool,
    #[structopt(short = "p", long = "publish", help = "Publish to crates.io on succesfull build")]
    publish: bool,
    #[structopt(long = "verbose", help = "Print debug info")]
    verbose: bool,
    #[structopt(short = "r", long = "release-config",
                help = "Provide an alternative path for the release config")]
    release_config: Option<String>,
    #[structopt(short = "s", long = "status-file",
                help = "Provide an alternative path for the status file")]
    status_file: Option<String>,
}

impl Opt {
    pub fn force_compile(&self) -> bool {
        self.force_compile
    }

    pub fn publish(&self) -> bool {
        self.publish
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn skip_dependencies(&self) -> bool {
        self.skip_dependencies
    }

    pub fn release_config(&self) -> Option<&str> {
        match self.release_config {
            Some(ref rc) => Some(rc),
            None => None,
        }
    }

    pub fn status_file(&self) -> Option<&str> {
        match self.status_file {
            Some(ref rc) => Some(rc),
            None => None,
        }
    }
}
