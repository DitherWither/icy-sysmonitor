use bytesize::ByteSize;
use iced::{
    subscription,
    widget::{column, row, ProgressBar, Text},
    Application, Command,
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

impl Application for IcySysMonitor {
    type Executor = iced::executor::Default;
    type Theme = iced::theme::Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                sys: System::new_all(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Icy System Monitor")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::UpdateInfo => {
                self.sys.refresh_all();
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            Text::new("Icy System Monitor").size(50),
            self.get_cpu_usage_element(),
            self.get_memory_usage_element(),
        ]
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .padding(20)
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // Get a random number
        let id = rand::random::<u64>();

        // The subscription that handles the updating of the system info
        // True is passed in as the second argument as our subscription doesn't have any state
        subscription::unfold(id, true, |_state| async move {
            // Wait 1 second before sending the next message
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            // Returns the UpdateInfo message and
            (Some(Message::UpdateInfo), true)
        })
    }
}

impl IcySysMonitor {
    /// Returns the widget storing the memory usage
    fn get_memory_usage_element(&self) -> iced::Element<Message> {
        // Convert the memory usage to a human readable format
        let used_memory = ByteSize(self.sys.used_memory());
        let total_memory = ByteSize(self.sys.total_memory());

        // The memory usage as a text widget
        let text_widget = Text::new(format!("Memory: {used_memory} / {total_memory}"));

        // The memory usage as a progress bar
        let progress_bar = ProgressBar::new(
            0.0..=100.0,
            self.sys.used_memory() as f32 / self.sys.total_memory() as f32 * 100.0,
        );
        row![text_widget, progress_bar].spacing(20).into()
    }

    /// Returns the widget storing the cpu usage
    fn get_cpu_usage_element(&self) -> iced::Element<Message> {
        // The column that will hold the cpu usage
        let mut cpu_column = column![].spacing(10).width(iced::Length::Fill);

        for (i, cpu) in self.sys.cpus().iter().enumerate() {
            // get the cpu usage as a percentage
            let cpu_usage = cpu.cpu_usage();

            // Progress bar widget storing the cpu usage
            let progress_bar = ProgressBar::new(0.0..=100.0, cpu_usage);

            // Round the cpu usage to 2 decimal places and left pad to 6 characters
            // So that the width is always the same
            let cpu_usage = format!("{:06.2}", cpu_usage);




            // Text widget storing the cpu usage
            let text_widget = Text::new(format!("CPU {i}: {cpu_usage}%"));

            // push the column containing the cpu usage
            cpu_column = cpu_column.push(row![text_widget, progress_bar].spacing(20).width(iced::Length::Fill));
        }

        cpu_column.into()
    }
}
