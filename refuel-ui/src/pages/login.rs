use iced::widget::button;
use iced::widget::button::Button;
use iced::{text_input, Checkbox, Column, Container, Font, Length, Row, Space, Text, TextInput};

use crate::style;

use super::{button_text, Message};
// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../../assets/Font Awesome 6 Free-Solid-900.otf"),
};

#[derive(Debug, Default)]
pub struct Login {
    state_username: text_input::State,
    username: String,
    state_password: text_input::State,
    password: String,
    action_login: button::State,
    remember_session: bool,
    action_new_user: button::State,
    action_reset_password: button::State,
}

impl Login {
    pub fn username_mut(&mut self) -> &mut String {
        &mut self.username
    }

    pub fn password_mut(&mut self) -> &mut String {
        &mut self.password
    }

    pub fn remember_session_mut(&mut self) -> &mut bool {
        &mut self.remember_session
    }

    pub fn view(&mut self) -> iced::Element<'_, Message> {
        Container::new(
            Column::new()
                .spacing(20)
                .padding(10)
                .push(Text::new("Welcome to Refuel").size(50))
                .push(Text::new("Please enter your credentials to continue...").size(32))
                .push(column().push(user_icon()).push(text_input(
                    "username",
                    &mut self.state_username,
                    &self.username,
                    Message::LoginUsernameChanged,
                )))
                .push(
                    column().push(password_icon()).push(
                        text_input(
                            "password",
                            &mut self.state_password,
                            &self.password,
                            Message::LoginPasswordChanged,
                        )
                        .password(),
                    ),
                )
                .push(
                    row()
                        .push(Checkbox::new(
                            self.remember_session,
                            "Remember me",
                            Message::LoginRememberMe,
                        ))
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Button::new(&mut self.action_login, button_text("Login"))
                                .style(style::Button::Primary)
                                .on_press(Message::LoginActionLogin),
                        )
                        .push(Space::with_width(Length::Fill))
                        .push(Space::with_width(Length::Fill)),
                )
                .push(
                    row()
                        .push(
                            Button::new(
                                &mut self.action_reset_password,
                                Text::new("Forgot Password?"),
                            )
                            .style(style::Button::HyperLink)
                            .on_press(Message::LoginActionResetPassword),
                        )
                        .push(Space::with_width(Length::Fill))
                        .push(
                            Button::new(&mut self.action_new_user, button_text("Create Account"))
                                .style(style::Button::Secondary)
                                .on_press(Message::LoginActionRegister),
                        ),
                ),
        )
        .padding(48)
        .style(style::Container::Base)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

fn icon(unicode: char) -> Text {
    Text::new(unicode.to_string())
        .font(ICONS)
        .width(Length::Units(20))
        .horizontal_alignment(iced::HorizontalAlignment::Center)
        .size(20)
}

fn user_icon() -> Text {
    icon('\u{f2bd}')
}

fn password_icon() -> Text {
    icon('\u{f084}')
}

fn column<'a>() -> Column<'a, Message> {
    Column::new().spacing(8)
}
fn row<'a>() -> Row<'a, Message> {
    Row::new().padding(20)
}

fn text_input<'a>(
    placeholder: &'a str,
    state: &'a mut text_input::State,
    value: &'a str,
    on_change: fn(String) -> Message,
) -> TextInput<'a, Message> {
    TextInput::new(state, placeholder, value, on_change)
        .style(style::TextInput::Regular)
        .size(24)
        .padding(8)
}
