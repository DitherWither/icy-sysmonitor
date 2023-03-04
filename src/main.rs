//! # A simple system monitor written in Rust using the iced toolkit
//!
//! This application is a simple system monitor that shows the cpu usage and memory usage of the system for now.
//!
//! The system information is retrieved using the sysinfo crate.
//! tokio is used as the async runtime for the iced application.
//!
//! It has been tested on Linux and Windows.
//!
//! It should in theory work on MacOS, but I have not tested it, and I don't have a Mac to test it on.

// Set the windows subsystem to windows to hide the console window on windows

#![windows_subsystem = "windows"]

mod config;
mod views;
mod window;

use iced::Application;

fn main() -> iced::Result {
    // Start the application
    // Default settings are used
    window::ApplicationWindow::run(iced::Settings::default())
}
