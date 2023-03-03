# A simple system monitor written in Rust using the iced toolkit

This application is a simple system monitor that shows the cpu usage and memory usage of the system for now.

The system information is retrieved using the `sysinfo` crate.
`tokio` is used as the async runtime for the iced application.

## Download

Releases will be available soon.

You can also download the automatic builds generated by github actions from the [builds](https://github.com/DitherWither/icy-sysmonitor/actions/workflows/build.yml) page.

## Building

To build the application, you need to have the rust toolchain installed. You can install it from [here](https://www.rust-lang.org/tools/install).

Once you have the toolchain installed, you can build the application by running the following command in the root directory of the project:

```bash
cargo build --release
```

The application doesn't have any external dependencies, and the executables
(`icy-sysmonitor` or `icy-sysmonitor.exe`) can be distibuted without any
additional files.

## TODO

- Add more system information
- Make the application look better
