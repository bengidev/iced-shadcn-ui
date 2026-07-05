# Theming

Components read design tokens from `ShadcnPalette` via iced's `.style()` closures. The palette maps shadcn/ui CSS variables to iced `Color` values for light and dark modes.

## Setup

`theme.rs` is copied on first `add`. Wire it into your app:

```rust
use ui::theme::{shadcn_dark_theme, shadcn_light_theme};

impl App {
    fn theme(state: &Self) -> Theme {
        if state.dark_mode {
            shadcn_dark_theme()
        } else {
            shadcn_light_theme()
        }
    }
}

fn main() -> iced::Result {
    application(App::default, update, view)
        .theme(App::theme)
        .run()
}
```

## Dark/light toggle

```rust
use iced::widget::toggler;

fn view(state: &App) -> Element<'_, Message> {
    toggler(state.dark_mode)
        .label("Dark mode")
        .on_toggle(Message::ToggleTheme)
        .into()
}
```

```rust
fn update(state: &mut App, message: Message) {
    match message {
        Message::ToggleTheme(dark) => state.dark_mode = dark,
        // ...
    }
}
```

## Token reference

| Token | shadcn CSS var | Usage |
|-------|----------------|-------|
| `background` | `--background` | App background |
| `foreground` | `--foreground` | Primary text |
| `card` | `--card` | Card surfaces |
| `card_foreground` | `--card-foreground` | Card text |
| `primary` | `--primary` | Primary actions, progress fill |
| `primary_foreground` | `--primary-foreground` | Text on primary |
| `secondary` | `--secondary` | Secondary surfaces |
| `secondary_foreground` | `--secondary-foreground` | Text on secondary |
| `muted` | `--muted` | Skeleton, avatar backgrounds |
| `muted_foreground` | `--muted-foreground` | Placeholder, subdued text |
| `accent` | `--accent` | Hover highlights |
| `accent_foreground` | `--accent-foreground` | Text on accent |
| `destructive` | `--destructive` | Destructive actions |
| `border` | `--border` | Borders, separators |
| `input` | `--input` | Input borders, switch track (off) |
| `ring` | `--ring` | Focus rings |
| `sidebar` | `--sidebar` | Sidebar background |
| `sidebar_foreground` | `--sidebar-foreground` | Sidebar text |
| `sidebar_primary` | `--sidebar-primary` | Active nav accent |
| `sidebar_primary_foreground` | `--sidebar-primary-foreground` | Text on sidebar primary |
| `sidebar_accent` | `--sidebar-accent` | Active nav item background |
| `sidebar_accent_foreground` | `--sidebar-accent-foreground` | Active nav text |
| `sidebar_border` | `--sidebar-border` | Sidebar border |
| `sidebar_ring` | `--sidebar-ring` | Sidebar focus ring |
| `radius_sm` | `--radius-sm` | Small radius (6px) |
| `radius_md` | `--radius-md` | Medium radius (8px) |
| `radius_lg` | `--radius-lg` | Large radius (10px) |

## Reading the palette in components

```rust
use ui::theme::palette;

.style(|theme| {
    let p = palette(theme);
    // use p.primary, p.border, p.radius_md, etc.
})
```

`palette()` returns `LIGHT` when the active theme is not dark, and `DARK` for `Theme::Dark`.

## Customizing colors

Edit the `LIGHT` and `DARK` constants in your copied `theme.rs`. Because you own the file, changes persist across `iced-shadcn` CLI updates.

Example — change the primary color in light mode:

```rust
pub const LIGHT: ShadcnPalette = ShadcnPalette {
    primary: color!(0x2563eb),           // blue-600
    primary_foreground: color!(0xffffff),
    // ... rest unchanged
};
```

For a different base palette, adjust the full struct to keep foreground/background contrast consistent.

## Base color and style

`iced-shadcn.toml` records `style = "new-york"` and `base_color = "neutral"`. v0.1 ships one template set (New York, neutral). Future releases may add more `registry/styles/` directories.
