# Release Manager
A tool to manage releasing crates with cross-compiled binaries.

In order to cross compile rust, you'll need to make sure you have targets added through rustup, and a native toolchain to handle compiling C dependencies and linking the binaries. Getting these tools set up is outside the scope of this project. See [rust-cross](https://github.com/japaric/rust-cross) for information about cross-compiling rust projects.

Release Manager currently handles the following targets:
 - `x86_64-pc-windows-gnu` configured as `[config.Windows.amd64]`
 - `i686-pc-windows-gnu` configured as `[config.Windows.i686]`
 - `x86_64-apple-darwin` configured as `[config.Apple.amd64]`
 - `x86_64-unknown-linux-gnu` configured as `[config.Linux.amd64]`
 - `arm-unknown-linux-gnueabihf` configured as `[config.Linux.armh]`
 - `armv7-unknown-linux-gnueabihf` configured as `[config.Linux.armv7h`
 - `aarch64-unknown-linux-gnu` configured as `[config.Linux.aarch64]`

Once you have your cross compile targets set up, create a `Release.toml`. By default, release-manager expects the release toml to be in your crate's directory. If you wish to keep this file somewhere else, you can pass the `-r` option to release-manager with a path to the config.
```Toml
release_path = "/home/asonix/Development/rust/releases"
license = "LICENSE"
readme = "README.md"

[config.Linux.aarch64]
libs = ["/usr/aarch64-linux-gnu/lib", "/home/asonix/Development/aarch64/lib"]
env = { OPENSSL_DIR = "/home/asonix/Development/aarch64", PKG_CONFIG_ALLOW_CROSS = "1" }

[config.Linux.armv7h]
libs = ["/usr/arm-linux-gnueabihf/lib", "/home/asonix/Development/armeabi/lib"]
env = { OPENSSL_DIR = "/home/asonix/Development/armeabi", PKG_CONFIG_ALLOW_CROSS = "1" }

[config.Linux.armh]
libs = ["/usr/arm-linux-gnueabihf/lib", "/home/asonix/Development/armeabi/lib"]
env = { OPENSSL_DIR = "/home/asonix/Development/armeabi", PKG_CONFIG_ALLOW_CROSS = "1" }

[config.Linux.amd64]
libs = []
env = {}

[config.Windows.amd64]
libs = ["/usr/x86_64-w64-mingw32/lib"]
env = { PKG_CONFIG_ALLOW_CROSS = "1" }
```

The help menu
```
$ release-manager
release-manager 0.1.0
Riley Trautman <asonix@tamu.edu>
A utility for creating release binaries for multiple platforms

USAGE:
    release-manager [FLAGS] [OPTIONS]

FLAGS:
    -f, --force      Force recompiling of succeeded builds
    -h, --help       Prints help information
    -p, --publish    Publish to crates.io on succesfull build
    -V, --version    Prints version information

OPTIONS:
    -r, --release-config <release_config>    Provide an alternative path for the release config
    -s, --status-file <status_file>          Provide an alternative path for the status file
```

When running release-manager, it will create a Status.toml file in your project's directory. If you wish to put this somewhere else, you can pass the `-s` option to release-manager. The status file is used to keep track of which builds have succeeded for which versions of the crate, and whether a crate has been published for this version.

Output directory structure
```
releases/release-manager/0.1.1 
[I] ➜  tree
.
├── aarch64-unknown-linux-gnu
│   ├── LICENSE
│   ├── README.md
│   └── release-manager
├── aarch64-unknown-linux-gnu.zip
├── arm-unknown-linux-gnueabihf
│   ├── LICENSE
│   ├── README.md
│   └── release-manager
├── arm-unknown-linux-gnueabihf.zip
├── armv7-unknown-linux-gnueabihf
│   ├── LICENSE
│   ├── README.md
│   └── release-manager
├── armv7-unknown-linux-gnueabihf.zip
├── x86_64-pc-windows-gnu
│   ├── LICENSE
│   ├── README.md
│   └── release-manager.exe
├── x86_64-pc-windows-gnu.zip
├── x86_64-unknown-linux-gnu
│   ├── LICENSE
│   ├── README.md
│   └── release-manager
└── x86_64-unknown-linux-gnu.zip

5 directories, 20 files

```

### License

Release Manager is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Release Manager is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of Release Manager

You should have received a copy of the GNU General Public License along with Release Manager If not, see http://www.gnu.org/licenses/.
