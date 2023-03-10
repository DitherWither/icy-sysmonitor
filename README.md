# A simple system monitor written in Rust using the iced toolkit

This application is a simple system monitor that shows the cpu usage and memory usage of the system for now.

The system information is retrieved using the `sysinfo` crate.
`tokio` is used as the async runtime for the iced application.

## Screenshots

![Screenshot of the Application](screenshots/window.png?raw=true "Screenshot of the Application")
![Screenshot of the Settings Page](screenshots/settings.png?raw=true "Screenshot of the Settings page")

## Download

Releases are available on the [releases](https://github.com/DitherWither/icy-sysmonitor/releases) page.

You can also download the automatic builds generated by github actions from the [builds](https://github.com/DitherWither/icy-sysmonitor/actions/workflows/build.yml) page.

## Platform support

The application should work on these platforms:

 - Windows
 - Linux
 - MacOS

However, it has only been tested on Linux and Windows as I don't have a mac.

## Usage

The application is very simple to use. Just run the executable and it will start monitoring the system.

## Configuration

The application can be configured from the settings page within the application itself.

The location of the config file is platform dependent.
 - On Linux it is stored in `~/.config/icy-sysmonitor/config.toml`
 - On Windows it is stored in `%APPDATA%\icy-sysmonitor\config.toml`
 - On MacOS should be stored in `~/Library/Application Support/io.github.DitherWither.icy-sysmonitor/config.toml`

## Building

To build the application from source, you need to have the rust toolchain installed. You can install it from [here](https://www.rust-lang.org/tools/install).

Once you have the toolchain installed, you can build the application by running the following command in the root directory of the project:

```bash
cargo build --release
```

The application doesn't have any external dependencies(for now), and the executables
(`icy-sysmonitor` or `icy-sysmonitor.exe`) can be distibuted without any
additional files.

## TODO

- Add more system information
- Make the application look better
