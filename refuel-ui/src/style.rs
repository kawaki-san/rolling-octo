use iced::{button, container, text_input, Background, Color};

pub enum Button {
    Primary,
    Secondary,
    HyperLink,
}

pub enum Container {
    Base,
}

pub enum TextInput {
    Regular,
}

impl text_input::StyleSheet for TextInput {
    fn active(&self) -> text_input::Style {
        match self {
            TextInput::Regular => {
                let rgb = hex_to_rgb("32302f");
                let border = hex_to_rgb("1d2021");
                text_input::Style {
                    background: Background::Color(Color::from_rgb(rgb.0, rgb.1, rgb.2)),
                    border_color: Color::from_rgb(border.0, border.1, border.2),
                    border_width: 0.5,
                    border_radius: 8.0,
                }
            }
        }
    }

    fn focused(&self) -> text_input::Style {
        match self {
            TextInput::Regular => {
                let rgb = hex_to_rgb("32302f");
                let border = hex_to_rgb("458588");
                text_input::Style {
                    background: Background::Color(Color::from_rgb(rgb.0, rgb.1, rgb.2)),
                    border_color: Color::from_rgb(border.0, border.1, border.2),
                    border_width: 0.5,
                    border_radius: 8.0,
                }
            }
        }
    }

    fn placeholder_color(&self) -> Color {
        match self {
            TextInput::Regular => {
                let rgb = hex_to_rgb("d4be98");
                Color::from_rgb(rgb.0, rgb.1, rgb.2)
            }
        }
    }

    fn value_color(&self) -> Color {
        match self {
            TextInput::Regular => {
                let rgb = hex_to_rgb("d4be98");
                Color::from_rgb(rgb.0, rgb.1, rgb.2)
            }
        }
    }

    fn selection_color(&self) -> Color {
        match self {
            TextInput::Regular => {
                let rgb = hex_to_rgb("89b482");
                Color::from_rgb(rgb.0, rgb.1, rgb.2)
            }
        }
    }
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        match self {
            Button::Primary => {
                let rgb = hex_to_rgb("458588");
                let tc = hex_to_rgb("d4be98");
                button::Style {
                    background: Some(Background::Color(Color::from_rgb(rgb.0, rgb.1, rgb.2))),
                    border_radius: 5.0,
                    text_color: Color::from_rgb(tc.0, tc.1, tc.2),
                    ..Default::default()
                }
            }
            Button::Secondary => {
                let bg = hex_to_rgb("d4be98");
                let fg = hex_to_rgb("282828");
                button::Style {
                    background: Some(Background::Color(Color::from_rgb(bg.0, bg.1, bg.2))),
                    border_radius: 5.0,
                    text_color: Color::from_rgb(fg.0, fg.1, fg.2),
                    ..Default::default()
                }
            }
            Button::HyperLink => {
                let fg = hex_to_rgb("458588");
                button::Style {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    text_color: Color::from_rgb(fg.0, fg.1, fg.2),
                    ..Default::default()
                }
            }
        }
    }

    fn hovered(&self) -> button::Style {
        match self {
            Button::Primary => {
                let rgb = hex_to_rgb("458588");
                let hint = hex_to_rgb("d4be98");
                button::Style {
                    background: Some(Background::Color(Color::from_rgb(rgb.0, rgb.1, rgb.2))),
                    border_color: Color::from_rgb(hint.0, hint.1, hint.2),
                    border_width: 1.5,
                    shadow_offset: iced::Vector::new(1.0, 2.0),
                    ..self.active()
                }
            }
            Button::Secondary => {
                let bg = hex_to_rgb("d4be98");
                let fg = hex_to_rgb("282828");
                let hint = hex_to_rgb("458588");
                button::Style {
                    background: Some(Background::Color(Color::from_rgb(bg.0, bg.1, bg.2))),
                    text_color: Color::from_rgb(fg.0, fg.1, fg.2),
                    border_color: Color::from_rgb(hint.0, hint.1, hint.2),
                    border_width: 1.0,
                    shadow_offset: iced::Vector::new(1.0, 2.0),
                    ..self.active()
                }
            }
            Button::HyperLink => {
                let fg = hex_to_rgb("7daea3");
                button::Style {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    text_color: Color::from_rgb(fg.0, fg.1, fg.2),
                    ..self.active()
                }
            }
        }
    }
}

fn hex_to_rgb(str: &str) -> (f32, f32, f32) {
    let base = 16;
    let z0 = i64::from_str_radix(&str[0..2], base).unwrap() as f32 / 255.0;
    let z1 = i64::from_str_radix(&str[2..4], base).unwrap() as f32 / 255.0;
    let z2 = i64::from_str_radix(&str[4..], base).unwrap() as f32 / 255.0;
    (z0, z1, z2)
}

impl container::StyleSheet for Container {
    fn style(&self) -> container::Style {
        let bg = hex_to_rgb("282828");
        let fg = hex_to_rgb("d4be98");
        match self {
            Container::Base => container::Style {
                background: Some(Background::Color(Color::from_rgb(bg.0, bg.1, bg.2))),
                text_color: Some(Color::from_rgb(fg.0, fg.1, fg.2)),
                ..Default::default()
            },
        }
    }
}
