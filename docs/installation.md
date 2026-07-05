# Installation

## Install the CLI

```bash
cargo install iced-shadcn-ui
```

This installs the `iced-shadcn` binary from crates.io (`iced-shadcn-ui` package). Component templates are fetched from GitHub at add-time — only the CLI is published.

## Add components to your project

From your iced 0.14 project root:

```bash
iced-shadcn add button
```

The first `add` will:

1. Create `iced-shadcn.toml` with defaults
2. Fetch `registry/registry.json` from the configured GitHub repo
3. Resolve transitive dependencies (`button` pulls in `theme` and `utils`)
4. Copy template files into your `ui_path`
5. Patch `mod.rs` with `pub mod` declarations
6. Merge iced 0.14 and required features into `Cargo.toml`

Existing files are skipped (not overwritten). Re-run `add` safely to pick up new components.

### Add multiple components

```bash
iced-shadcn add card input checkbox
```

### List available components

Requires `iced-shadcn.toml` (created by the first `add`):

```bash
iced-shadcn list
```

### Preview changes without writing

Requires `iced-shadcn.toml`:

```bash
iced-shadcn diff button
```

## Configuration (`iced-shadcn.toml`)

Created automatically on first `add`:

```toml
style = "new-york"
base_color = "neutral"
ui_path = "src/ui"
iced_version = "0.14"
registry_url = "https://github.com/bengidev/iced-shadcn-ui"
registry_branch = "main"
```

| Field | Description |
|-------|-------------|
| `style` | Template style set. v0.1 supports `new-york`. |
| `base_color` | Color palette base. v0.1 uses `neutral`. |
| `ui_path` | Directory where component `.rs` files are written. Default: `src/ui`. |
| `iced_version` | iced version merged into `Cargo.toml`. Default: `0.14`. |
| `registry_url` | GitHub repo URL used to fetch `registry.json` and templates. |
| `registry_branch` | Git branch used for raw registry fetches. Default: `main`. |

### Custom `ui_path`

To install components under a different module path:

```toml
ui_path = "src/widgets"
```

The CLI writes files to that directory and patches `src/widgets/mod.rs`. Import with:

```rust
use widgets::*;
```

## Project layout after first `add`

```
my-app/
├── iced-shadcn.toml
├── Cargo.toml
└── src/
    ├── main.rs
    └── ui/
        ├── mod.rs       # auto-patched
        ├── theme.rs     # ShadcnPalette + light/dark themes
        ├── utils.rs     # border helpers
        └── button.rs    # first added component
```

## Cargo.toml changes

`add` ensures your `Cargo.toml` includes iced with the features required by added components (typically `advanced`, `tokio`, and `wgpu`). Adding `icons`, `button`, or `sidebar` also adds [`lucide-icons`](https://crates.io/crates/lucide-icons) for [Lucide](https://lucide.dev/) icon support.

You do not need to add a dependency on `iced-shadcn` — components are copied source, not a library crate.

### Lucide font (required for icons)

If your project uses icons, load the bundled font once at startup:

```rust
use ui::icons::LUCIDE_FONT_BYTES;

iced::application(App::default, update, view)
    .font(LUCIDE_FONT_BYTES)
    .run()
```

See [icons.md](components/icons.md) for usage.

## Using components

After `iced-shadcn add button`:

```rust
use ui::button::{button, ButtonVariant};

fn view() -> Element<'_, Message> {
    button("Save")
        .variant(ButtonVariant::Default)
        .on_press(Message::Save)
        .into_button()
        .into()
}
```

Wire up theming — see [theming.md](theming.md).

## Showcase

Clone this repo and run the reference app:

```bash
cargo run -p showcase
```
