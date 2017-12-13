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

extern crate toml;
extern crate serde;
extern crate zip;
extern crate walkdir;
extern crate structopt;
extern crate env_logger;

#[macro_use]
extern crate log;

extern crate release_manager;

use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::exit;
use std::env;
use toml::Value;
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::FileOptions;
use structopt::StructOpt;
use log::{LogLevelFilter, LogRecord};
use env_logger::LogBuilder;

use release_manager::{Config, Error, Opt, StatusWrapper};
use release_manager::{parse_toml, publish, table_str};

fn zip_directory(result_path: &str, zip_contents_path: &str, zip_name: &str) -> Result<(), Error> {
    if !Path::new(zip_contents_path).is_dir() {
        return Err(Error::ZipPath);
    }

    let path_str = format!("{}/{}", result_path, zip_name);
    let path = Path::new(&path_str);
    let file = File::create(&path)?;

    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let walkdir = WalkDir::new(zip_contents_path);

    for dent in walkdir.into_iter().filter_map(|e| e.ok()) {
        let path = dent.path();
        let name = path.strip_prefix(Path::new(zip_contents_path))?
            .to_str()
            .ok_or(Error::ZipPath)?;

        if path.is_file() {
            zip.start_file(name, options)?;
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
        }
    }

    zip.finish()?;

    Ok(())
}

fn do_main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut log_builder = LogBuilder::new();
    log_builder.format(|record: &LogRecord| format!("{}", record.args()));
    log_builder.filter(
        Some("release_manager"),
        if opt.verbose() {
            LogLevelFilter::Debug
        } else {
            LogLevelFilter::Info
        },
    );
    if let Ok(ref rust_log) = env::var("RUST_LOG") {
        log_builder.parse(rust_log);
    }
    log_builder.init().unwrap();

    let rc = opt.release_config();
    let release_path = if let Some(ref rc) = rc {
        Path::new(rc)
    } else {
        Path::new("Release.toml")
    };

    let cargo_path = Path::new("Cargo.toml");

    let config: Config = parse_toml(&release_path)?;
    let cargo: Value = parse_toml(&cargo_path)?;

    let sf = opt.status_file();
    let status_path = if let Some(ref sf) = sf {
        Path::new(sf)
    } else {
        Path::new("Status.toml")
    };

    let mut status = StatusWrapper::new(&status_path);
    let _ = status.read();

    let package = cargo.get("package").ok_or(Error::PackageMissing)?;
    let name = table_str(package, "name", Error::NameMissing)?;
    let version = table_str(package, "version", Error::VersionMissing)?;

    if status.is_published(version) && opt.publish() {
        return Err(Error::RePublish);
    }

    let full_release_path_string = format!("{}/{}/{}", &config.release_path, &name, version);
    let full_release_path = Path::new(&full_release_path_string);

    fs::create_dir_all(&full_release_path)?;

    let targets = config.targets();

    let dir = env::current_dir()?;
    let dir_str = dir.to_str().ok_or(Error::PathString)?;

    status.clear_missing_targets(
        version,
        targets
            .iter()
            .map(|t| t.target_str())
            .collect::<Vec<_>>()
            .as_ref(),
    );
    status.write()?;

    for target in targets {
        if !opt.force_compile() && !status.needs_compile(target.target_str(), version) {
            info!("Skipping: {}, already compiled", target.target_str());
            continue;
        }

        let proc_status = target.compile(version, &mut status)?;

        if proc_status.success() {
            let build_path = format!(
                "{}/target/{}/release/{}",
                dir_str,
                target.target_str(),
                &name
            );

            let license_path = format!("{}/{}", dir_str, &config.license);
            let readme_path = format!("{}/{}", dir_str, &config.readme);

            let dest_dir = format!("{}/{}", &full_release_path_string, target.target_str());

            fs::create_dir_all(&dest_dir)?;
            fs::copy(license_path, format!("{}/{}", dest_dir, &config.license))?;
            fs::copy(readme_path, format!("{}/{}", dest_dir, &config.readme))?;

            if fs::metadata(&build_path).is_ok() {
                let dest_path = format!("{}/{}", dest_dir, &name);

                fs::copy(build_path, dest_path)?;
            } else {
                let build_path = format!("{}.exe", build_path);

                if fs::metadata(&build_path).is_ok() {
                    let dest_path =
                        format!(
                            "{}/{}.exe",
                            dest_dir,
                            &name,
                            );

                    fs::copy(build_path, dest_path)?;
                }
            }

            zip_directory(
                &full_release_path_string,
                &dest_dir,
                &format!("{}.zip", target.target_str()),
            )?;
        }
    }

    status.write()?;

    if !status.all_clear(version) {
        return Err(Error::FailedBuilds);
    }

    if opt.publish() {
        publish().map(|exit_status| if exit_status.success() {
            status.publish(version);
        })?;
        status.write()?;
    }

    Ok(())
}

fn main() {
    exit(match do_main() {
        Ok(_) => 0,
        Err(e) => {
            error!("{}", e);
            1
        }
    });
}
