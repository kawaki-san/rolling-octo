mod login;

use iced::{executor, Application, Text};
pub use login::*;
use tracing::{error, info, trace, warn};

#[derive(Debug, Default)]
pub struct Refuel {
    page: Page,
}

impl Default for Page {
    fn default() -> Self {
        Self::Login(Login::default())
    }
}

#[derive(Debug)]
enum Page {
    Login(Login),
}

impl ToString for Page {
    fn to_string(&self) -> String {
        match &self {
            Page::Login(_) => "Login",
        }
        .to_owned()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    LoginUsernameChanged(String),
    LoginPasswordChanged(String),
    LoginRememberMe(bool),
    LoginActionLogin,
    LoginActionRegister,
    LoginActionResetPassword,
}

impl Application for Refuel {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), iced::Command::none())
    }

    fn title(&self) -> String {
        let app_name = env!("CARGO_PKG_NAME");
        let app_name = format!("{}{}", &app_name[0..1].to_uppercase(), &app_name[1..]);
        format!("{} - {}", app_name, &self.page.to_string())
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match &mut self.page {
            Page::Login(ref mut state) => match message {
                Message::LoginUsernameChanged(username) => {
                    trace!("username changed");
                    *state.username_mut() = username;
                }
                Message::LoginPasswordChanged(password) => {
                    trace!("password changed");
                    *state.password_mut() = password;
                }
                Message::LoginRememberMe(_) => {
                    trace!("checkbox toggled");
                    *state.remember_session_mut() = !*state.remember_session_mut()
                }
                Message::LoginActionLogin => {
                    info!("login clicked");
                    //TODO
                }
                Message::LoginActionRegister => {
                    warn!("register clicked");
                    //TODO
                }
                Message::LoginActionResetPassword => {
                    error!("reset clicked");
                    //TODO
                }
            },
        }
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        match &mut self.page {
            Page::Login(ref mut state) => state.view(),
        }
    }
}

pub fn button_text(text: &str) -> Text {
    Text::new(format!("   {text}   "))
}
