# Icons (Lucide)

iced-shadcn uses [Lucide](https://lucide.dev/) icons via the [`lucide-icons`](https://crates.io/crates/lucide-icons) crate — the official Rust port with iced widget support.

## Setup

Add the icons module:

```bash
iced-shadcn add icons
```

This copies `src/ui/icons.rs` and adds `lucide-icons` to your `Cargo.toml`.

Load the Lucide font when starting your app (required once):

```rust
use ui::icons::LUCIDE_FONT_BYTES;

fn main() -> iced::Result {
    iced::application(App::default, update, view)
        .font(LUCIDE_FONT_BYTES)
        .run()
}
```

## Usage

```rust
use ui::icons::{self, Icon};

// Themed icon (foreground color)
icons::themed_icon(Icon::Search, 16.0)

// Parse by kebab-case name from lucide.dev
icons::themed_icon_named("arrow-right", 16.0)

// Icon-only button
ui::button::icon_button(Icon::Plus).on_press(Message::Add)

// Label + leading icon
ui::button::button_with_icon(Icon::Download, "Download")
```

Browse all icon names at [lucide.dev/icons](https://lucide.dev/icons).

## Sidebar icons

```rust
use ui::icons::Icon;
use ui::sidebar::{self, SidebarItem};

sidebar::sidebar(
    false,
    &[SidebarItem {
        label: "Home",
        icon: Icon::Home,
        active: true,
    }],
    |_| Message::Navigate(0),
);
```
