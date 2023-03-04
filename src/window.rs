use iced::{
    time,
    widget::{button, column, row, Text},
    Application, Command,
};
use std::time::Duration;
use sysinfo::{System, SystemExt};

use crate::{
    config,
    views::settings::{SettingsMessage, SettingsState},
};

/// The application struct that implements the Application trait
///
/// This struct contains all the data that is needed to run the application
/// and implements the Application for the UI.
pub struct ApplicationWindow {
    /// The system object to get system info from
    ///
    /// This object is created in the new function and is used to get system info
    /// in the update function
    pub sys: sysinfo::System,

    /// The current page of the main window
    ///
    /// This variable is used to store the current page of the main window.
    pub page: MainWindowPage,

    /// The config object to store the settings
    ///
    /// This object is used to store the settings of the application.
    /// The settings are stored in a config file in the user's home directory.
    /// The location of the config file is platform dependent.
    ///  * On Linux it is stored in `~/.config/icy-sysmonitor/config.toml`
    ///  * On Windows it is stored in `%APPDATA%\icy-sysmonitor\config.toml`
    ///  * On MacOS should be stored in `~/Library/Application Support/io.github.DitherWither.icy-sysmonitor/config.toml`
    ///
    /// The config file is created if it does not exist.
    /// The default settings are used if the config file does not exist.
    /// The default settings are:
    /// ```toml
    /// # The update interval in seconds
    /// update_interval = 1
    /// ```
    /// The settings can be changed by the user in the settings page.
    /// Should be loaded in the new function
    pub config: config::Config,
}

/// The message enum for the application to communicate with itself
///
/// The communication should be handled by the update function which is called automatically
/// by iced whenever a message is sent
#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    /// ApplicationMessage to update the system info
    ///
    /// This message is sent to the application every second to update the system info.
    UpdateInfo,

    /// ApplicationMessage to open the settings view
    ///
    /// This message is sent to the application when the settings button is pressed.
    OpenSettings,

    /// ApplicationMessage to open the home view
    ///
    /// This message is sent to the application when the home button is pressed.
    OpenHome,

    /// ApplicationMessage when the settings page is updated
    ///
    /// This message is sent to the application when the settings page is updated.
    /// This message should be handled by the settings page's update function.
    SettingsPageUpdated(SettingsMessage),
}

/// The enum for the pages of the main window of the application
///
/// This enum is used to store the current page of the main window.
/// The main window currently only has two pages: the home page and the settings page.
pub enum MainWindowPage {
    /// The home page of the main window
    ///
    /// This page currently contains the system info widgets and the settings button
    Home,

    /// The settings page of the main window
    ///
    /// TODO: Rename to just Settings
    SettingsPage(SettingsState),
}

/// The implementation of the application
impl Application for ApplicationWindow {
    type Executor = iced::executor::Default;
    type Theme = iced::theme::Theme; // TODO: Add dark theme
    type Flags = ();
    type Message = ApplicationMessage;

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                // Create a new system object to get system info
                sys: System::new_all(),

                // Set the current page to the home page
                page: MainWindowPage::Home,

                // Load the config file
                config: config::Config::load(),
            },
            // Return a command to do nothing as we don't need to do anything else
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Icy System Monitor".to_string()
    }

    fn update(&mut self, message: ApplicationMessage) -> iced::Command<ApplicationMessage> {
        match message {
            // Update the system info
            ApplicationMessage::UpdateInfo => {
                self.sys.refresh_all();
            }

            // Open the settings page
            ApplicationMessage::OpenSettings => {
                self.page = MainWindowPage::SettingsPage(SettingsState::new(&self.config));
            }

            // Open the home page
            ApplicationMessage::OpenHome => {
                self.page = MainWindowPage::Home;
            }

            // Update the settings page
            ApplicationMessage::SettingsPageUpdated(message) => self.settings_page_update(&message),
        }

        // Return a command to do nothing as we don't need to do anything else
        Command::none()
    }

    fn view(&self) -> iced::Element<ApplicationMessage> {
        // The header of the main window
        let header = self.get_header();

        // The main content of the main window
        let main_content = match &self.page {
            MainWindowPage::Home => self.home_page_view(),
            MainWindowPage::SettingsPage(state) => self.settings_page_view(state).map(|message| {
                // Map the message to the application's message
                ApplicationMessage::SettingsPageUpdated(message)
            }),
        };

        // Create the main window
        column![header, main_content]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
    }

    fn subscription(&self) -> iced::Subscription<ApplicationMessage> {
        // Send a message every second to update the system info in the update function
        // The update interval is stored in the config file
        time::every(Duration::from_millis(self.config.update_interval))
            .map(|_| ApplicationMessage::UpdateInfo)
    }
}

/// The parts of the window that are shared between the pages
impl ApplicationWindow {
    /// Returns the header of the main window
    ///
    /// This function returns the header of the main window which contains the title and the settings button
    ///
    /// # Example
    ///
    /// ```
    /// // Get the header
    /// let header = self.get_header();
    /// // Roughly looks like:
    /// //                      ------- -----------
    /// // Icy System Monitor   |Home | |Settings |
    /// //                      ------- -----------
    /// ```
    fn get_header(&self) -> iced::Element<ApplicationMessage> {
        // Create the title
        let title = Text::new("Icy System Monitor").size(50);

        // Create the home button
        let home_button = button(Text::new("Home").size(20)).on_press(ApplicationMessage::OpenHome);

        // Create the settings button
        let settings_button =
            button(Text::new("Settings").size(20)).on_press(ApplicationMessage::OpenSettings);

        // Create the header
        row![title, home_button, settings_button]
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .padding(20)
            .spacing(20)
            .into()
    }
}
