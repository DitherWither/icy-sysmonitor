//! # A simple system monitor written in Rust using the iced toolkit
//! 
//! This application is a simple system monitor that shows the cpu usage and memory usage of the system for now.
//! 
//! The system information is retrieved using the sysinfo crate.
//! tokio is used as the async runtime for the iced application. 

// Set the windows subsystem to windows to hide the console window on windows
#![windows_subsystem = "windows"]

use std::time::Duration;

use bytesize::ByteSize;
use iced::{
    time,
    widget::{column, row, ProgressBar, Text},
    Application, Command,
};
use sysinfo::{CpuExt, System, SystemExt};

fn main() -> iced::Result {
    // Start the application
    // Default settings are used
    IcySysMonitor::run(iced::Settings::default())
}

/// The message enum for the application to communicate with itself
///
/// The communication should be handled by the update function which is called automatically
/// by iced whenever a message is sent
#[derive(Debug, Clone)]
enum Message {
    /// Message to update the system info
    ///
    /// This message is sent to the application every second to update the system info.
    UpdateInfo,
}

/// The application struct that implements the Application trait
///
/// This struct contains all the data that is needed to run the application
/// and implements the Application for the UI.
struct IcySysMonitor {
    /// The system object to get system info from
    ///
    /// This object is created in the new function and is used to get system info
    /// in the update function
    sys: System,
}

/// The implementation of the application
impl Application for IcySysMonitor {
    type Executor = iced::executor::Default;
    type Theme = iced::theme::Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                // Create a new system object to get system info
                sys: System::new_all(),
            },
            // Return a command to do nothing as we don't need to do anything else
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Icy System Monitor".to_string()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            // Update the system info
            Message::UpdateInfo => {
                self.sys.refresh_all();
            }
        }

        // Return a command to do nothing as we don't need to do anything else
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        // Create the title
        let title = Text::new("Icy System Monitor").size(50);

        // Get the cpu usage panel
        let cpu_usage = self.get_cpu_usage_panel();

        // Get the memory usage widget
        let memory_usage = self.get_memory_usage_element();

        // Create the main application view
        column![title, cpu_usage, memory_usage]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // Send a message every second to update the system info
        time::every(Duration::from_secs(1)).map(|_| Message::UpdateInfo)
    }
}

impl IcySysMonitor {
    /// Returns the widget storing the memory usage
    ///
    /// This function returns a row containing the memory usage as a text widget
    /// and a progress bar widget
    ///
    /// # Example
    ///
    /// ```
    /// let memory_usage = self.get_memory_usage_element();
    /// // Roughly looks like this:
    /// // Memory: 1.00 GiB / 7.79 GiB [=====================>  ]
    /// ```
    fn get_memory_usage_element(&self) -> iced::Element<Message> {
        // Convert the memory usage to a human readable format
        let used_memory = ByteSize(self.sys.used_memory());
        let total_memory = ByteSize(self.sys.total_memory());

        // The memory usage as a text widget
        let text_widget = Text::new(format!("Memory: {used_memory} / {total_memory}"));

        // The memory usage as a progress bar
        let progress_bar = ProgressBar::new(
            0.0..=(self.sys.total_memory() as f32),
            self.sys.used_memory() as f32,
        );
        row![text_widget, progress_bar].spacing(20).into()
    }

    /// Returns the widget storing the cpu usage of all CPUs
    ///
    /// This function returns a column containing the cpu usage of all CPUs
    /// as a row of text and progress bar widgets
    ///
    /// # Example
    ///
    /// ```
    /// let cpu_usage = self.get_cpu_usage_panel();   
    /// // Roughly looks like this:
    /// //
    /// // CPU 0: 050.00% [=====================>  ]
    /// // CPU 1: 100.00% [========================]
    /// // CPU 2: 000.00% [                        ]
    /// // CPU 3: 012.35% [===>                    ]
    /// // CPU 4: 100.00% [========================]
    /// ```
    fn get_cpu_usage_panel(&self) -> iced::Element<Message> {
        // The column that will hold the cpu usage
        let mut cpu_column = column![].spacing(10).width(iced::Length::Fill);

        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            // Push the cpu usage of a single cpu to the column
            cpu_column = cpu_column.push(self.get_cpu_usage_row(i as i32, cpu.cpu_usage()));
        }

        cpu_column.into()
    }

    /// Returns the widget storing the cpu usage of a single cpu
    ///
    /// This function returns a row containing the cpu usage as a text widget
    /// and a progress bar widget
    ///
    /// # Arguments
    ///
    /// * `cpu_num` - The number of the cpu (0, 1, 2, etc.)
    /// * `cpu_usage` - The cpu usage of the cpu as a float between 0 and 100
    ///
    /// # Example
    ///
    /// ```
    /// // The comments roughly describe the output
    /// let cpu_usage = self.get_cpu_usage_row(0, 50.0);   // CPU 0: 050.00% [=====================>  ]
    /// let cpu_usage = self.get_cpu_usage_row(1, 100.0);  // CPU 1: 100.00% [========================]
    /// let cpu_usage = self.get_cpu_usage_row(2, 0.0);    // CPU 2: 000.00% [                        ]
    /// let cpu_usage = self.get_cpu_usage_row(3, 12.345); // CPU 3: 012.35% [===>                    ]
    /// let cpu_usage = self.get_cpu_usage_row(4, 99.999); // CPU 4: 100.00% [========================]
    /// ```
    fn get_cpu_usage_row(&self, cpu_num: i32, cpu_usage: f32) -> iced::Element<Message> {
        // Progress bar widget storing the cpu usage
        let progress_bar = ProgressBar::new(0.0..=100.0, cpu_usage);

        // Round the cpu usage to 2 decimal places and left pad to 6 characters
        // So that the width is always the same
        let cpu_usage = format!("{:06.2}", cpu_usage);

        // Text widget storing the cpu usage
        let text_widget = Text::new(format!("CPU {cpu_num}: {cpu_usage}%"));

        row![text_widget, progress_bar].spacing(20).into()
    }
}
