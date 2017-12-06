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

extern crate release_manager;

use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::env;
use toml::Value;
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::FileOptions;
use structopt::StructOpt;

use release_manager::{Config, Error, Opt, StatusWrapper};
use release_manager::{parse_toml, publish, table_str};

fn zip_directory(result_path: &str, zip_contents_path: &str, zip_name: &str) -> Result<(), Error> {
    if !Path::new(zip_contents_path).is_dir() {
        return Err(Error::ZipPathError);
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
            .ok_or(Error::ZipPathError)?;

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

fn main() {
    let opt = Opt::from_args();

    let rc = opt.release_config();
    let release_path = if let Some(ref rc) = rc {
        Path::new(rc)
    } else {
        Path::new("Release.toml")
    };

    let cargo_path = Path::new("Cargo.toml");

    let config: Config = parse_toml(&release_path).unwrap();
    let cargo: Value = parse_toml(&cargo_path).unwrap();

    let sf = opt.status_file();
    let status_path = if let Some(ref sf) = sf {
        Path::new(sf)
    } else {
        Path::new("Status.toml")
    };

    let mut status = StatusWrapper::new(&status_path);
    let _ = status.read();

    let package = cargo.get("package").ok_or(Error::PackageMissing).unwrap();
    let name = table_str(package, "name", Error::NameMissing).unwrap();
    let version = table_str(package, "version", Error::VersionMissing).unwrap();

    if status.is_published(version) && opt.publish() {
        panic!("Publish flag set for version that has already been published");
    }

    let full_release_path_string = format!("{}/{}/{}", &config.release_path, &name, version);
    let full_release_path = Path::new(&full_release_path_string);

    fs::create_dir_all(&full_release_path).expect("Unable to create release dir");

    let targets = config.targets();

    let dir = env::current_dir().unwrap();
    let dir_str = dir.to_str().unwrap();

    for target in targets {
        if !opt.force_compile() && !status.needs_compile(target.target_str(), version) {
            println!("Skipping: {}, already compiled", target.target_str());
            continue;
        }

        let proc_status = target.compile(version, &mut status).expect(
            "Unable to start compile",
        );

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

            fs::create_dir_all(&dest_dir).expect("Unable to create destination directory");
            fs::copy(license_path, format!("{}/{}", dest_dir, &config.license))
                .expect("Unable to copy LICENSE");
            fs::copy(readme_path, format!("{}/{}", dest_dir, &config.readme))
                .expect("Unable to copy README");

            if fs::metadata(&build_path).is_ok() {
                let dest_path = format!("{}/{}", dest_dir, &name);

                fs::copy(build_path, dest_path).expect("Unable to copy binary");
            } else {
                let build_path = format!("{}.exe", build_path);

                if fs::metadata(&build_path).is_ok() {
                    let dest_path =
                        format!(
                            "{}/{}.exe",
                            dest_dir,
                            &name,
                            );

                    fs::copy(build_path, dest_path).expect("Unable to copy binary");
                }
            }

            zip_directory(
                &full_release_path_string,
                &dest_dir,
                &format!("{}.zip", target.target_str()),
            ).unwrap();
        }
    }

    status.write().unwrap();

    if !status.all_clear() {
        panic!("Some builds failed, exiting");
    }

    if opt.publish() {
        publish()
            .map(|exit_status| if exit_status.success() {
                status.publish(version);
            })
            .unwrap();
        status.write().unwrap();
    }
}
