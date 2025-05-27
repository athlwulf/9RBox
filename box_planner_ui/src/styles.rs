use iced::widget::text;
use iced::widget::{button, container};
use iced::{Background, Color, Theme};

// Grayscale color palette as functions
pub fn lightest() -> Color { Color::from_rgb8(0xF2, 0xF2, 0xF2) }
pub fn light() -> Color { Color::from_rgb8(0xBF, 0xBF, 0xBF) }
pub fn medium() -> Color { Color::from_rgb8(0x73, 0x73, 0x73) }
pub fn dark() -> Color { Color::from_rgb8(0x59, 0x59, 0x59) }
pub fn darkest() -> Color { Color::from_rgb8(0x0D, 0x0D, 0x0D) }

// Highlighted container style for grid boxes
pub struct BoxHighlightStyle;
impl container::StyleSheet for BoxHighlightStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba8(0xBF, 0xBF, 0xBF, 0.3))), // 0.3 alpha as float
            border: iced::Border {
                color: medium(),
                width: 2.5,
                radius: iced::border::Radius::from(4.0),
            },
            shadow: Default::default(),
            text_color: None,
        }
    }
}


pub struct PrimaryButton;
impl button::StyleSheet for PrimaryButton {
    type Style = Theme;
    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(medium())),
            border: iced::Border {
                radius: iced::border::Radius::from(5.0),
                ..Default::default()
            },
            text_color: lightest(),
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(dark())),
            border: iced::Border {
                radius: iced::border::Radius::from(5.0),
                ..Default::default()
            },
            text_color: lightest(),
            ..Default::default()
        }
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(darkest())),
            border: iced::Border {
                radius: iced::border::Radius::from(5.0),
                ..Default::default()
            },
            text_color: lightest(),
            ..Default::default()
        }
    }
}

// Style for unhighlighted 9-box containers
pub struct DefaultBoxStyle;
impl container::StyleSheet for DefaultBoxStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(light())),
            border: iced::Border {
                color: medium(),
                width: 2.0,
                radius: iced::border::Radius::from(4.0),
            },
            shadow: Default::default(),
            text_color: None,
        }
    }
}

// Style for the root application background
pub struct AppBackground;
impl container::StyleSheet for AppBackground {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(lightest())),
            ..Default::default()
        }
    }
}
// Dynamic style for grid boxes with specific background colors
pub struct ThemedBoxStyle(pub Color);
impl container::StyleSheet for ThemedBoxStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0)),
            border: iced::Border {
                color: Color::from_rgba(
                    self.0.r * 0.8,
                    self.0.g * 0.8,
                    self.0.b * 0.8,
                    self.0.a,
                ),
                width: 2.0,
                radius: iced::border::Radius::from(6.0),
            },
            shadow: iced::Shadow {
                offset: iced::Vector::new(1.0, 2.0),
                color: Color::from_rgba8(0, 0, 0, 0.15),
                blur_radius: 4.0,
            },
            text_color: None,
        }
    }
}
// Style for employee cards (tile-like appearance)
pub struct CardStyle;
impl container::StyleSheet for CardStyle {
    type Style = Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::WHITE)),
            border: iced::Border {
                color: medium(),
                width: 1.0,
                radius: iced::border::Radius::from(6.0),
            },
            shadow: iced::Shadow {
                offset: iced::Vector::new(0.0, 1.0),
                color: Color::from_rgba8(0, 0, 0, 0.1),
                blur_radius: 3.0,
            },
            text_color: Some(Color::BLACK),
        }
    }
}
pub mod text_styles {
    use super::*;

    pub fn heading(text: &str) -> text::Text<'_> {
        text::Text::new(text)
            .size(24)
            .style(Color::BLACK)
    }

    pub fn subheading(text: &str) -> text::Text<'_> {
        text::Text::new(text)
            .size(20)
            .style(dark())
    }

    pub fn body(text: &str) -> text::Text<'_> {
        text::Text::new(text)
            .size(16)
            .style(medium())
    }

    pub fn light_body(text: &str) -> text::Text<'_> {
        text::Text::new(text)
            .size(16)
            .style(dark())
    }
}