use crate::window::{ApplicationMessage, ApplicationWindow};
use bytesize::ByteSize;
use iced::widget::{column, row, ProgressBar, Text};

use sysinfo::{CpuExt, SystemExt};

// TODO: Make this a seperate struct instead of an impl block
impl ApplicationWindow {
    /// Returns the home page panel of the main window
    ///
    /// This function returns the home page panel of the main window which contains the system info widgets
    pub fn home_page_view(&self) -> iced::Element<ApplicationMessage> {
        // Get the cpu usage panel
        let cpu_usage = self.get_cpu_usage_panel();

        // Get the memory usage widget
        let memory_usage = self.get_memory_usage_element();

        // Create the main application view
        column![cpu_usage, memory_usage]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
    }

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
    fn get_memory_usage_element(&self) -> iced::Element<ApplicationMessage> {
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
    fn get_cpu_usage_panel(&self) -> iced::Element<ApplicationMessage> {
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
    /// # Examplewidth
    ///
    /// ```
    /// // The comments roughly describe the output
    /// let cpu_usage = self.get_cpu_usage_row(0, 50.0);   // CPU 0: 050.00% [=====================>  ]
    /// let cpu_usage = self.get_cpu_usage_row(1, 100.0);  // CPU 1: 100.00% [========================]
    /// let cpu_usage = self.get_cpu_usage_row(2, 0.0);    // CPU 2: 000.00% [                        ]
    /// let cpu_usage = self.get_cpu_usage_row(3, 12.345); // CPU 3: 012.35% [===>                    ]
    /// let cpu_usage = self.get_cpu_usage_row(4, 99.999); // CPU 4: 100.00% [========================]
    /// ```
    fn get_cpu_usage_row(&self, cpu_num: i32, cpu_usage: f32) -> iced::Element<ApplicationMessage> {
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
