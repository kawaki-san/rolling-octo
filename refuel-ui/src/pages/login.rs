use iced::{button, text_input, Button, Checkbox, Column, Container, Text, TextInput};

use super::Message;

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
                .push(Text::new("Welcome to Refuel").size(48))
                .push(Text::new("Please enter your credentials to continue").size(36))
                .push(
                    TextInput::new(
                        &mut self.state_username,
                        "username",
                        &self.username,
                        Message::LoginUsernameChanged,
                    )
                    .size(24)
                    .padding(8),
                )
                .push(
                    TextInput::new(
                        &mut self.state_password,
                        "password",
                        &self.password,
                        Message::LoginPasswordChanged,
                    )
                    .size(24)
                    .padding(8)
                    .password(),
                )
                .push(Checkbox::new(
                    self.remember_session,
                    "Remember me",
                    Message::LoginRememberMe,
                ))
                .push(
                    Button::new(&mut self.action_login, Text::new("Login"))
                        .on_press(Message::LoginActionLogin),
                )
                .push(
                    Button::new(
                        &mut self.action_reset_password,
                        Text::new("Forgot Password?"),
                    )
                    .on_press(Message::LoginActionResetPassword),
                )
                .push(
                    Button::new(&mut self.action_new_user, Text::new("Create Account"))
                        .on_press(Message::LoginActionRegister),
                ),
        )
        .into()
    }
}
