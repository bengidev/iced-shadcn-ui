//! [Lucide](https://lucide.dev/) icons for iced via the `lucide-icons` crate.

pub use lucide_icons::{Icon, LUCIDE_FONT_BYTES};

use crate::ui::theme::palette;
use iced::widget::text;
use iced::{Color, Element, Theme};

const LUCIDE_FONT: iced::Font = iced::Font::with_name("lucide");

pub fn icon<'a, Message: 'a>(icon: Icon, size: f32) -> Element<'a, Message, Theme> {
    text(icon.unicode().to_string())
        .font(LUCIDE_FONT)
        .size(size)
        .into()
}

pub fn themed_icon<'a, Message: 'a>(icon: Icon, size: f32) -> Element<'a, Message, Theme> {
    text(icon.unicode().to_string())
        .font(LUCIDE_FONT)
        .size(size)
        .style(|theme| text::Style {
            color: Some(palette(theme).foreground),
        })
        .into()
}

pub fn icon_with_color<'a, Message: 'a>(
    icon: Icon,
    size: f32,
    color: Color,
) -> Element<'a, Message, Theme> {
    text(icon.unicode().to_string())
        .font(LUCIDE_FONT)
        .size(size)
        .style(move |theme| text::Style {
            color: Some(color),
        })
        .into()
}

pub fn themed_icon_named<'a, Message: 'a>(
    name: &str,
    size: f32,
) -> Option<Element<'a, Message, Theme>> {
    Icon::try_from(name)
        .ok()
        .map(|icon| themed_icon(icon, size))
}
