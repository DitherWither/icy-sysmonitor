use crate::{
    config::Config,
    window::{ApplicationWindow, MainWindowPage},
};

use iced::widget::{button, column, row, slider, Text};

/// Enum for communication inside the settings page
///
/// This enum is used to communicate between the different widgets in the settings page.
/// The communication should be handled by the update function which is called automatically
/// by iced whenever a message is sent
#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /// Message to update the update interval
    ///
    /// This message is sent to the settings page when the update interval text input is updated.
    UpdateIntervalChanged(f64),

    /// Message to save the settings
    ///
    /// This message is sent to the settings page when the save button is pressed.
    /// This message should save the settings to disk.
    SaveSettings,

    /// Message to cancel the changes to settings
    ///
    /// This message is sent to the settings page when the cancel button is pressed.
    /// This message should discard the changes to the settings.
    CancelSettings,

    /// Message to reset the settings to default
    ///
    /// This message is sent to the settings page when the reset button is pressed.
    /// This message should reset the settings to the default settings.
    ResetSettings,
}

/// The settings page's state
pub struct SettingsState {
    /// The update interval field's value in milliseconds
    ///
    /// This variable is used to store the value of the update interval field.
    /// It is used to update the update interval field when the user types in it.
    /// It is also used to update the config object when the save button is pressed.
    update_interval: u64,
}

impl SettingsState {
    /// Creates a new settings state
    pub fn new(config: &Config) -> Self {
        Self {
            update_interval: config.update_interval,
        }
    }
}

/// TODO: Seperate the settings page into a different struct instead of an impl block
impl ApplicationWindow {
    /// Returns the settings page view of the main window
    pub fn settings_page_view(&self, state: &SettingsState) -> iced::Element<SettingsMessage> {
        let title: iced::Element<_> = Text::new("Settings page").size(50).into();

        // The update interval row
        let update_interval_row = self.get_update_interval_row(state);

        // The buttons row
        let buttons_row = self.get_settings_page_buttons_row();

        column![title, update_interval_row, buttons_row]
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(20)
            .spacing(20)
            .align_items(iced::Alignment::Center)
            .into()
    }

    pub fn settings_page_update(&mut self, message: &SettingsMessage) {
        let state = match &mut self.page {
            MainWindowPage::Settings(state) => state,
            _ => {
                // TODO: Make it show a dialog instead of printing to the terminal
                eprintln!(
                    "ApplicationMessage::SettingsPageUpdated was sent when the settings page \
                        was not open, this should not happen!"
                );
                eprintln!("Please report this bug at https://github.com/DitherWither/icy-sysmonitor/issues");
                eprintln!("Continuing as if nothing happened...");
                return;
            }
        };

        match message {
            SettingsMessage::UpdateIntervalChanged(value) => {
                // Value is in seconds, convert to milliseconds
                state.update_interval = (*value * 1000.0) as u64;
            }
            SettingsMessage::SaveSettings => {
                self.config.update_interval = state.update_interval;
                self.config.save();
            }
            SettingsMessage::CancelSettings => {
                state.update_interval = self.config.update_interval;
            }
            SettingsMessage::ResetSettings => {
                self.config = Config::default();
                self.config.save();

                // This will update the settings page to show the default settings
                // As the config is reloaded when canceling the settings
                // TODO: This is a bit hacky, maybe find a better way to do this
                self.settings_page_update(&SettingsMessage::CancelSettings);
            }
        }
    }
}

/// The widgets used in the settings page
impl ApplicationWindow {
    /// Returns the button row for saving and canceling the settings
    ///
    /// This function returns the row that contains the save and cancel buttons.
    /// The save button should save the settings to disk.
    /// The cancel button should discard the changes to the settings.
    ///
    /// # Example
    ///
    /// ```
    /// let buttons_row = self.get_settings_page_buttons_row();
    /// // This roughly looks like this:
    /// // [Cancel] [Save settings]
    /// ```
    fn get_settings_page_buttons_row(&self) -> iced::Element<SettingsMessage> {
        // Button to save the settings
        let save_button = button(Text::new("Save")).on_press(SettingsMessage::SaveSettings);

        // Button to cancel the changes to the settings
        // TODO: Make the cancel button red
        let cancel_button = button(Text::new("Cancel")).on_press(SettingsMessage::CancelSettings);

        // Button to reset the settings to default
        // TODO: Make the reset button red
        let reset_button =
            button(Text::new("Reset to Default")).on_press(SettingsMessage::ResetSettings);

        row![save_button, cancel_button, reset_button]
            .spacing(10)
            .into()
    }

    /// Returns the row that contains the update interval input slider and the label
    /// that shows the current value of the input slider
    ///
    /// This function returns the row that contains the update interval input slider and the label
    /// that shows the current value of the input slider.
    ///
    /// # Example
    ///
    /// ```
    /// let update_interval_row = self.get_update_interval_row();
    /// // This roughly looks like this:
    /// // Update interval: [-||-----------] 1.0 seconds
    fn get_update_interval_row(&self, state: &SettingsState) -> iced::Element<SettingsMessage> {
        // Title for the update interval input slider
        let update_interval_title = Text::new("Update interval");

        // Update interval in seconds f64
        let update_interval_seconds = state.update_interval as f64 / 1000.0;

        // Create the update interval input slider
        let update_interval_input = slider(
            0.1..=10.0,
            update_interval_seconds,
            SettingsMessage::UpdateIntervalChanged,
        )
        .step(0.1);

        // Text label that shows the current value of the update interval input slider
        let update_interval_value_label = Text::new(format!(
            "{:.1} seconds",
            state.update_interval as f64 / 1000.0
        ));

        // The row that contains the update interval input slider and the label
        let update_interval_row = row![
            update_interval_title,
            update_interval_input,
            update_interval_value_label
        ]
        .spacing(10);

        update_interval_row.into()
    }
}
