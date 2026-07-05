mod pages;
mod ui;

use iced::widget::{column, row, text, toggler};
use iced::{application, Element, Theme};
use lucide_icons::Icon;
use ui::icons::LUCIDE_FONT_BYTES;
use ui::theme::{shadcn_dark_theme, shadcn_light_theme};

#[derive(Default)]
struct App {
    dark_mode: bool,
    page: Page,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Page {
    #[default]
    Buttons,
    Inputs,
    Layout,
    Data,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme(bool),
    SelectPage(Page),
    PageAction,
}

fn main() -> iced::Result {
    application(App::default, update, view)
        .theme(App::theme)
        .title("iced-shadcn showcase")
        .font(LUCIDE_FONT_BYTES)
        .run()
}

impl App {
    fn theme(state: &Self) -> Theme {
        if state.dark_mode {
            shadcn_dark_theme()
        } else {
            shadcn_light_theme()
        }
    }
}

fn update(state: &mut App, message: Message) {
    match message {
        Message::ToggleTheme(v) => state.dark_mode = v,
        Message::SelectPage(p) => state.page = p,
        Message::PageAction => {}
    }
}

fn view(state: &App) -> Element<'_, Message> {
    let nav_items = [
        ui::sidebar::SidebarItem {
            label: "Buttons",
            icon: Icon::MousePointerClick,
            active: state.page == Page::Buttons,
        },
        ui::sidebar::SidebarItem {
            label: "Inputs",
            icon: Icon::TextCursorInput,
            active: state.page == Page::Inputs,
        },
        ui::sidebar::SidebarItem {
            label: "Layout",
            icon: Icon::LayoutGrid,
            active: state.page == Page::Layout,
        },
        ui::sidebar::SidebarItem {
            label: "Data",
            icon: Icon::Table,
            active: state.page == Page::Data,
        },
    ];

    let nav = ui::sidebar::sidebar(
        false,
        &nav_items,
        |idx| {
            Message::SelectPage(match idx {
                0 => Page::Buttons,
                1 => Page::Inputs,
                2 => Page::Layout,
                _ => Page::Data,
            })
        },
    );

    let content: Element<_> = match state.page {
        Page::Buttons => pages::buttons::view().map(|()| Message::PageAction),
        Page::Inputs => pages::inputs::view().map(|()| Message::PageAction),
        Page::Layout => pages::layout::view().map(|()| Message::PageAction),
        Page::Data => pages::data::view().map(|()| Message::PageAction),
    };

    column![
        row![
            text("iced-shadcn showcase").size(24),
            toggler(state.dark_mode)
                .label("Dark mode")
                .on_toggle(Message::ToggleTheme),
        ]
        .spacing(20),
        row![nav, content].spacing(16),
    ]
    .padding(20)
    .into()
}
