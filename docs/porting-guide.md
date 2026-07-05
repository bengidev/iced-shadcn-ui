# Porting guide

How to translate shadcn/ui (React + Tailwind + CSS variables) into iced-shadcn components.

## Mental model

| shadcn/ui | iced-shadcn |
|-----------|-------------|
| Copy component into `components/ui/` | `iced-shadcn add <name>` copies `.rs` into `ui_path` |
| Tailwind classes | iced `.style()` closures + `ShadcnPalette` tokens |
| CSS variables (`--primary`) | `ShadcnPalette` fields (`primary`) |
| Radix primitives | iced built-in widgets (`button`, `text_input`, `checkbox`, `table`, …) |
| React state (`useState`) | User's Elm `App` state + `Message` enum |
| `className` variants | Rust enums (`ButtonVariant`) + builder methods |

Components are **not** a widget library crate. Each file is self-contained source the user can edit.

## CSS variables → `ShadcnPalette`

shadcn defines tokens in `globals.css`:

```css
:root {
  --background: 0 0% 100%;
  --primary: 0 0% 9%;
  --radius: 0.5rem;
}
```

In iced-shadcn, map these to `ShadcnPalette` in `theme.rs`:

```rust
pub const LIGHT: ShadcnPalette = ShadcnPalette {
    background: color!(0xffffff),
    primary: color!(0x171717),
    radius_md: 8.0,
    // ...
};
```

Use `palette(theme)` inside every `.style()` closure so components respond to dark/light mode.

## React component → iced builder

**shadcn Button (React):**

```tsx
<Button variant="outline" size="sm" onClick={save}>
  Save
</Button>
```

**iced-shadcn Button (Rust):**

```rust
button("Save")
    .variant(ButtonVariant::Outline)
    .size(ButtonSize::Sm)
    .on_press(Message::Save)
    .into_button()
```

Pattern:

1. Factory function (`button`, `input`, `badge`) returns a builder struct or `Element`
2. Builder methods set variant, size, callbacks
3. `into_button()` / `.into()` produces the final iced widget

## The `.style()` pattern

shadcn applies Tailwind per variant. In iced, use a `match` on variant and `button::Status`:

```rust
fn styled(variant: ButtonVariant, p: ShadcnPalette, status: button::Status) -> button::Style {
    let (bg, fg, border_color) = match variant {
        ButtonVariant::Default => (p.primary, p.primary_foreground, p.primary),
        ButtonVariant::Outline => (p.background, p.foreground, p.border),
        // ...
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: fg,
        border: border(border_color, width, p.radius_md),
        ..Default::default()
    }
}
```

Attach with `.style(move |theme, status| styled(variant, palette(theme), status))`.

## Simple vs composed components

**Simple** — style an iced built-in:

- `input` → `text_input` + focus ring on `Status::Focused`
- `checkbox` → `checkbox` + `checkbox::Style`
- `switch` → `toggler` + track/thumb colors
- `badge`, `label`, `separator`, `skeleton`, `progress`, `avatar` → `container`, `text`, `rule`, `progress_bar`

**Composed** — combine multiple widgets:

- `card` → `container` + `column` + bordered surface
- `sidebar` → `container` + `mouse_area` nav items + sidebar tokens
- `scroll_area` → `scrollable` wrapper
- `table` → iced 0.14 `table::table` with column definitions

Only implement a custom `Widget` when built-ins are insufficient (not needed for v0.1).

## State and messages

shadcn components manage local UI state via React hooks. In iced, **all state lives in the user's `App`**:

```rust
struct App {
    email: String,
    accept_terms: bool,
}

enum Message {
    EmailChanged(String),
    TermsToggled(bool),
}

// view passes state into components:
input("Email", &state.email, Message::EmailChanged)
checkbox("Accept terms", state.accept_terms, Message::TermsToggled)
```

## Registry template placeholders

Templates use `{{ui_module}}` for the Rust module path (derived from `ui_path`). The CLI substitutes this at render time so `use {{ui_module}}::theme::palette` becomes `use crate::ui::theme::palette`.

## Checklist for a new component

1. Study the [shadcn source](https://ui.shadcn.com/) for variants and token usage
2. Pick the closest iced widget(s)
3. Define variant enums matching shadcn options
4. Implement `.style()` using `palette(theme)` and `utils::border`
5. Add a `.rs.template` under `registry/styles/new-york/`
6. Register in `registry/registry.json` with `registryDependencies: ["theme", "utils"]`
7. Add a showcase page section and `docs/components/<name>.md`
8. Run `cargo test -p iced-shadcn` and `cargo run -p showcase`

See [contributing.md](contributing.md) for the full workflow.
