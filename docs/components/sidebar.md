# Sidebar

Application sidebar with collapsible rail and Lucide navigation icons.

## Install

```bash
iced-shadcn add sidebar
```

Pulls in `icons`, `theme`, and `utils` automatically.

## Usage

```rust
use lucide_icons::Icon;
use ui::sidebar::{sidebar, SidebarItem};

let items = [
    SidebarItem {
        label: "Home",
        icon: Icon::Home,
        active: true,
    },
    SidebarItem {
        label: "Settings",
        icon: Icon::Settings,
        active: false,
    },
];

sidebar(false, &items, |idx| Message::SelectPage(idx))
```

| Parameter | Description |
|-----------|-------------|
| `collapsed` | `true` for icon-only rail (56px), `false` for expanded (240px) |
| `items` | Slice of `SidebarItem` with `label`, `icon`, and `active` |
| `on_select` | Callback with item index on press |

Browse icons at [lucide.dev/icons](https://lucide.dev/icons).

## Example

```rust
row![
    sidebar(
        state.sidebar_collapsed,
        &nav_items,
        Message::SelectPage,
    ),
    main_content,
]
```

Uses dedicated sidebar tokens (`sidebar`, `sidebar_accent`, `sidebar_border`, etc.). v0.1 is desktop-first — no mobile sheet overlay.
