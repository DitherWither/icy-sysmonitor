use iced::{
    widget::{button, column, Text},
    Sandbox,
};
use sysinfo::{CpuExt, System, SystemExt};

fn main() -> iced::Result {
    // Start the application
    MainWindow::run(iced::Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    UpdateInfo,
}

// The main window
struct MainWindow {
    sys: System,
}

impl Sandbox for MainWindow {
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
        // The column that will hold the cpu usage
        let mut cpu_column = column![];

        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            // Add the cpu usage to the column
            cpu_column = cpu_column.push(Text::new(format!("CPU {}: {}%", i, cpu.cpu_usage())));
        }

        let used_memory = self.sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_memory = self.sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;

        // Round the used_memory and total_memory to 2 decimal places
        let used_memory = format!("{:.2}", used_memory);
        let total_memory = format!("{:.2}", total_memory);

        // The element storing the memory usage
        let memory_usage = Text::new(format!("Memory Usage: {used_memory} GiB / {total_memory} GiB"));

        column![
            Text::new("Icy System Monitor").size(50),
            cpu_column,
            memory_usage,
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
