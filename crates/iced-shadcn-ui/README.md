# iced-shadcn-ui

CLI for adding [shadcn/ui](https://ui.shadcn.com)-inspired components to [iced](https://github.com/iced-rs/iced) 0.14 projects.

Components are **copied as Rust source** into your repo — no runtime widget crate dependency. You own and edit the files.

Install the **`iced-shadcn-ui`** package; the binary is named **`iced-shadcn`**.

## Install

```bash
cargo install iced-shadcn-ui
```

Templates are fetched from GitHub at add-time. Only this CLI is published to crates.io.

## Quick start

From your iced 0.14 project root:

```bash
iced-shadcn add button
```

The first `add` will:

1. Create `iced-shadcn.toml` with defaults
2. Fetch the component registry from GitHub
3. Resolve dependencies (`button` pulls in `theme`, `utils`, and `icons`)
4. Copy `.rs` templates into `src/ui/` (configurable)
5. Patch `src/ui/mod.rs` with `pub mod` declarations
6. Merge `iced` and required features into `Cargo.toml`

Add `mod ui;` to `src/main.rs` (or `lib.rs`) if it is not already present.

## Commands

| Command | Description |
|---------|-------------|
| `iced-shadcn add <name>...` | Add one or more components |
| `iced-shadcn list` | List available components |
| `iced-shadcn diff <name>` | Preview template diff without writing files |

```bash
iced-shadcn add card badge input
iced-shadcn list
iced-shadcn diff button
```

`list` and `diff` require an existing `iced-shadcn.toml` (created by the first `add`).

## Components (v0.1)

| Component | CLI name |
|-----------|----------|
| Button | `button` |
| Badge | `badge` |
| Label | `label` |
| Separator | `separator` |
| Skeleton | `skeleton` |
| Progress | `progress` |
| Input | `input` |
| Checkbox | `checkbox` |
| Switch | `switch` |
| Card | `card` |
| Avatar | `avatar` |
| Scroll area | `scroll-area` |
| Table | `table` |
| Sidebar | `sidebar` |
| Icons (Lucide) | `icons` |

Existing files are skipped on re-run. Use `diff` to compare your copy against upstream templates.

## Configuration

`iced-shadcn.toml` is created on the first `add`:

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
| `style` | Template style set (`new-york` in v0.1) |
| `base_color` | Palette base (`neutral` in v0.1) |
| `ui_path` | Output directory for component files |
| `iced_version` | iced version merged into `Cargo.toml` |
| `registry_url` | GitHub repo for registry and templates |
| `registry_branch` | Git branch for raw fetches (default: `main`) |

### Local registry override

For development or offline use:

```bash
export ICED_SHADCN_REGISTRY_DIR=/path/to/registry
iced-shadcn add button
```

## Usage example

```rust
mod ui;

use ui::button::{button, ButtonVariant};

fn view() -> iced::Element<'_, Message> {
    button("Save")
        .variant(ButtonVariant::Default)
        .on_press(Message::Save)
        .into_button()
        .into()
}
```

For icons, load the Lucide font once at startup:

```rust
use ui::icons::LUCIDE_FONT_BYTES;

iced::application(App::default, update, view)
    .font(LUCIDE_FONT_BYTES)
    .run()
```

## Full documentation

- [Installation guide](https://github.com/bengidev/iced-shadcn-ui/blob/main/docs/installation.md)
- [Theming](https://github.com/bengidev/iced-shadcn-ui/blob/main/docs/theming.md)
- [Porting guide](https://github.com/bengidev/iced-shadcn-ui/blob/main/docs/porting-guide.md)
- [Contributing](https://github.com/bengidev/iced-shadcn-ui/blob/main/docs/contributing.md)
- [Component reference](https://github.com/bengidev/iced-shadcn-ui/tree/main/docs/components)
- [Showcase app](https://github.com/bengidev/iced-shadcn-ui/tree/main/examples/showcase)

## License

MIT — see [LICENSE](https://github.com/bengidev/iced-shadcn-ui/blob/main/LICENSE).
