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
use std::fmt;

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
    ZipPath,
    /// Path could not be converted to string
    PathString,
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
    /// Cannot publish already published crate
    RePublish,
    /// Some builds failed
    FailedBuilds,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::IO(ref err) => write!(f, "IO Error: {}", err),
            Error::TomlRead(ref err) => write!(f, "Error reading TOML: {}", err),
            Error::TomlWrite(ref err) => write!(f, "Error writing TOML: {}", err),
            Error::Zip(ref err) => write!(f, "Error making .zip: {}", err),
            Error::ZipPath => write!(f, "Path for zip file is invalid"),
            Error::PathString => write!(f, "Path could not be converted to string"),
            Error::InvalidTarget => write!(f, "Supplied target is not supported"),
            Error::PackageMissing => write!(f, "Crate is missing [package] section"),
            Error::NameMissing => write!(f, "Crate is missing the 'name' parameter"),
            Error::VersionMissing => write!(f, "Crate is missing the 'version' parameter"),
            Error::NotString => write!(f, "Expected a toml::Value::String, got something else"),
            Error::NotTable => write!(f, "Expected a toml::Value::Table, got somethign else"),
            Error::RePublish => write!(f, "Cannot re-publish an already published crate"),
            Error::FailedBuilds => write!(f, "Some builds failed"),
        }
    }
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
        Error::ZipPath
    }
}
