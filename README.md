# A simple system monitor written in Rust using the iced toolkit

This application is a simple system monitor that shows the cpu usage and memory usage of the system for now.

The system information is retrieved using the `sysinfo` crate.
`tokio` is used as the async runtime for the iced application.

## Building

To build the application, you need to have the rust toolchain installed. You can install it from [here](https://www.rust-lang.org/tools/install).

Once you have the toolchain installed, you can build the application by running the following command in the root directory of the project:

```bash
cargo build --release
```

## TODO

- [ ] Add more system information
- [ ] Add proper packaging for the application
- [ ] Make the application look better
- [ ] Replace the "full" feature of tokio with the individual features that are required
