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

use toml::de::Error as TomlReadError;
use toml::ser::Error as TomlWriteError;
use zip::result::ZipError;
use std::io::Error as IOError;
use std::path::StripPrefixError;

#[derive(Debug)]
pub enum Error {
    /// Error performing io
    IO(IOError),
    /// Error reading from TOML strings
    TomlRead(TomlReadError),
    /// Error writing to TOML strings
    TomlWrite(TomlWriteError),
    /// Error while zipping file
    Zip(ZipError),
    /// Zip placement path doesn't exist
    ZipPathError,
    /// Target specified is not supported
    InvalidTarget,
    /// Target crate has no [package] section
    PackageMissing,
    /// Target crate has no name
    NameMissing,
    /// Target crate has no version
    VersionMissing,
    /// Expected a string (when parsing toml)
    NotString,
    /// Expected a table (when parsing toml)
    NotTable,
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::IO(err)
    }
}

impl From<TomlReadError> for Error {
    fn from(err: TomlReadError) -> Self {
        Error::TomlRead(err)
    }
}

impl From<TomlWriteError> for Error {
    fn from(err: TomlWriteError) -> Self {
        Error::TomlWrite(err)
    }
}

impl From<ZipError> for Error {
    fn from(err: ZipError) -> Self {
        Error::Zip(err)
    }
}

impl From<StripPrefixError> for Error {
    fn from(_: StripPrefixError) -> Self {
        Error::ZipPathError
    }
}
