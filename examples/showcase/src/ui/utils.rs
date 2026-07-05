//! Shared helpers for shadcn/ui iced components.

use iced::Border;

pub fn border(color: iced::Color, width: f32, radius: f32) -> Border {
    Border {
        color,
        width,
        radius: radius.into(),
    }
}
