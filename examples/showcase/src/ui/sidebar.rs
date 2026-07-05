use crate::ui::icons::{self, Icon};
use crate::ui::theme::palette;
use iced::widget::{column, container, mouse_area, row, text, Column};
use iced::{Background, Element, Length, Theme};

#[derive(Clone, Copy)]
pub struct SidebarItem {
    pub label: &'static str,
    pub icon: Icon,
    pub active: bool,
}

pub fn sidebar<'a, Message: Clone + 'a>(
    collapsed: bool,
    items: &[SidebarItem],
    on_select: impl Fn(usize) -> Message + Clone + 'a,
) -> Element<'a, Message, Theme> {
    let width = if collapsed { 56.0 } else { 240.0 };
    let entries: Column<'a, Message> = Column::with_children(
        items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let msg = on_select.clone()(idx);
                let active = item.active;
                let content: Element<'a, Message> = if collapsed {
                    icons::themed_icon(item.icon, 18.0)
                } else {
                    row![
                        icons::themed_icon(item.icon, 16.0),
                        text(item.label),
                    ]
                    .spacing(10)
                    .align_y(iced::Alignment::Center)
                    .into()
                };
                let entry = container(content)
                    .width(Length::Fill)
                    .padding(10)
                    .style(move |theme| {
                        let p = palette(theme);
                        container::Style {
                            background: Some(Background::Color(if active {
                                p.sidebar_accent
                            } else {
                                p.sidebar
                            })),
                            text_color: Some(p.sidebar_foreground),
                            ..Default::default()
                        }
                    });
                mouse_area(entry).on_press(msg).into()
            })
            .collect::<Vec<_>>(),
    )
    .spacing(4);

    container(entries)
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .style(|theme| {
            let p = palette(theme);
            container::Style {
                background: Some(Background::Color(p.sidebar)),
                border: iced::Border {
                    color: p.sidebar_border,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
}
