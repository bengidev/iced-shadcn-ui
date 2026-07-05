//! shadcn/ui Button for iced.
//!
//! ```rust
//! ui::button("Save")
//!     .variant(ButtonVariant::Default)
//!     .on_press(Message::Save)
//! ```

use crate::ui::theme::{palette, ShadcnPalette};
use crate::ui::utils::border;
use iced::widget::{button as iced_button, Button};
use iced::{Background, Element, Theme};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

pub struct ShadcnButton<'a, Message> {
    label: Element<'a, Message, Theme>,
    variant: ButtonVariant,
    size: ButtonSize,
    on_press: Option<Message>,
}

impl<'a, Message: Clone + 'a> ShadcnButton<'a, Message> {
    pub fn new(label: impl Into<Element<'a, Message, Theme>>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            on_press: None,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn into_button(self) -> Button<'a, Message, Theme> {
        let variant = self.variant;
        let padding = match self.size {
            ButtonSize::Sm => [8, 12],
            ButtonSize::Lg => [12, 24],
            ButtonSize::Icon => [8, 8],
            ButtonSize::Default => [10, 16],
        };
        let mut btn = iced_button(self.label)
            .padding(padding)
            .style(move |theme, status| styled(variant, palette(theme), status));
        if let Some(message) = self.on_press {
            btn = btn.on_press(message);
        }
        btn
    }
}

pub fn button<'a, Message: Clone + 'a>(
    label: impl Into<Element<'a, Message, Theme>>,
) -> ShadcnButton<'a, Message> {
    ShadcnButton::new(label)
}

fn styled(
    variant: ButtonVariant,
    p: ShadcnPalette,
    status: iced::widget::button::Status,
) -> iced::widget::button::Style {
    use iced::widget::button::Status::{Disabled, Hovered, Pressed};
    let (bg, fg, border_color) = match variant {
        ButtonVariant::Default => (p.primary, p.primary_foreground, p.primary),
        ButtonVariant::Destructive => (p.destructive, p.foreground, p.destructive),
        ButtonVariant::Outline => (p.background, p.foreground, p.border),
        ButtonVariant::Secondary => (p.secondary, p.secondary_foreground, p.secondary),
        ButtonVariant::Ghost | ButtonVariant::Link => (p.background, p.foreground, p.background),
    };
    let background = match (variant, status) {
        (ButtonVariant::Ghost, Hovered) | (ButtonVariant::Ghost, Pressed) => p.accent,
        (ButtonVariant::Link, _) => p.background,
        (_, Hovered) | (_, Pressed) if variant != ButtonVariant::Link => bg,
        _ => bg,
    };
    let opacity = match status {
        Disabled => 0.5,
        _ => 1.0,
    };
    iced::widget::button::Style {
        background: Some(Background::Color(background.scale_alpha(opacity))),
        text_color: fg.scale_alpha(opacity),
        border: border(
            border_color.scale_alpha(opacity),
            if variant == ButtonVariant::Outline { 1.0 } else { 0.0 },
            p.radius_md,
        ),
        ..iced::widget::button::Style::default()
    }
}
