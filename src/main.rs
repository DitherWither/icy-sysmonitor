use iced::{
    widget::{button, column, Text},
    Sandbox,
};
use sysinfo::{CpuExt, System, SystemExt};

fn main() -> iced::Result {
    // Start the application
    IcySysMonitor::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    UpdateInfo,
}

// The main window
struct IcySysMonitor {
    sys: System,
}

impl Sandbox for IcySysMonitor {
    type Message = Message;

    fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    fn title(&self) -> String {
        String::from("Icy System Monitor")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::UpdateInfo => {
                self.sys.refresh_all();
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            Text::new("Icy System Monitor").size(50),
            self.get_cpu_usage_element(),
            self.get_memory_usage_element(),
            button(Text::new("Update")).on_press(Message::UpdateInfo),
        ]
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .padding(20)
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }
}

impl IcySysMonitor {
    /// Returns the widget storing the memory usage
    fn get_memory_usage_element(&self) -> iced::Element<Message> {
        // Get the memory usage
        let used_memory = self.sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_memory = self.sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        // Round the used_memory and total_memory to 2 decimal places
        let used_memory = format!("{:.2}", used_memory);
        let total_memory = format!("{:.2}", total_memory);

        // Return the memory usage as a text widget
        Text::new(format!("Memory: {used_memory} / {total_memory} GB")).into()
    }

    /// Returns the widget storing the cpu usage
    fn get_cpu_usage_element(&self) -> iced::Element<Message> {
        // The column that will hold the cpu usage
        let mut cpu_column = column![];

        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            // get the cpu usage as a percentage
            let cpu_usage = cpu.cpu_usage();

            // Round the cpu usage to 2 decimal places
            let cpu_usage = format!("{:.2}", cpu_usage);

            // Add the cpu usage to the column
            cpu_column = cpu_column.push(Text::new(format!("CPU {i}: {cpu_usage}%")));
        }

        cpu_column.into()
    }
}
