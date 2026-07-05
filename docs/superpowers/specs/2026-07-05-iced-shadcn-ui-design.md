# iced-shadcn-ui Design Spec

**Date:** 2026-07-05  
**Status:** Approved  
**Author:** bengidev  

## Summary

Port [shadcn/ui](https://github.com/shadcn-ui/ui) design patterns into [iced](https://github.com/iced-rs/iced) 0.14 via a **shadcn-style CLI** that copies self-contained Rust component source into the user's project. The CLI (`iced-shadcn`) is published to crates.io; component templates are fetched from GitHub at add-time.

This is a design-system translation, not a line-by-line React port. Components follow iced's Elm Architecture: state lives in the user's app; copied files provide builders, style functions, and layout helpers.

## Goals

- Give iced developers a polished, shadcn-quality component set without waiting for an official iced ecosystem
- Preserve shadcn's "you own the code" philosophy — no runtime widget crate dependency
- Ship a usable v0.1 with 14 components (foundation + layout) and dark/light theming
- Publish CLI to crates.io; host registry and docs on GitHub (`bengidev/iced-shadcn-ui`)

## Non-Goals (v0.1)

- Overlay components (`dialog`, `popover`, `dropdown-menu`, `tooltip`, `select`, `tabs`)
- Mobile sheet behavior for sidebar
- mdBook documentation site
- Snapshot/visual regression tests
- Supporting iced 0.13 alongside 0.14

## Design Decisions

| Area | Decision |
|------|----------|
| Distribution model | shadcn-style CLI — copy source into user project |
| iced version | 0.14.x |
| v0.1 component scope | 14 components: foundation + layout |
| Theming | Dark + light, runtime toggle |
| Shared code | Fully self-contained copies — no core crate dependency |
| Bootstrap workflow | `add` only; first `add` copies theme/utils if missing |
| Registry source | Fetched from GitHub at add-time |
| Output path | Configurable via `iced-shadcn.toml`, default `src/ui/` |
| Repo structure | Monorepo: CLI crate + registry + showcase + docs |
| License | MIT (consistent with shadcn/ui) |

## Architecture

### Repository Layout

```
iced-shadcn-ui/
├── crates/
│   └── iced-shadcn/              # CLI binary (published to crates.io)
├── registry/
│   ├── registry.json             # component manifest
│   ├── schema.json               # JSON schema for registry
│   └── styles/
│       └── new-york/             # shadcn New York style templates
│           ├── theme.rs.template
│           ├── utils.rs.template
│           ├── button.rs.template
│           └── ...
├── examples/
│   └── showcase/                 # runnable demo of all v0.1 components
├── docs/
│   ├── installation.md
│   ├── theming.md
│   ├── porting-guide.md
│   ├── contributing.md
│   └── components/               # one page per component
├── CHANGELOG.md
├── LICENSE
└── README.md
```

### Published vs Copied Artifacts

| Artifact | Location | Purpose |
|----------|----------|---------|
| `iced-shadcn` CLI | crates.io | Install via `cargo install iced-shadcn` |
| Component templates | GitHub `registry/` | Fetched by CLI at add-time |
| Theme + utilities + widgets | User's project | Self-contained, user-owned source |

### User Project After First `add`

```
my-app/
├── iced-shadcn.toml
├── Cargo.toml
└── src/
    ├── main.rs
    └── ui/                       # default; configurable
        ├── mod.rs                # auto-patched by CLI
        ├── theme.rs              # shadcn tokens → iced Color (light + dark)
        ├── utils.rs              # radius helpers, variant enums
        └── button.rs             # first added component
```

### Theme System

shadcn CSS variables become a Rust `ShadcnPalette` struct with light and dark variants. Components use iced `.style()` closures reading from the active palette. Runtime toggle via iced's `theme()` function:

```rust
fn theme(state: &App) -> Theme {
    if state.dark_mode {
        shadcn_dark_theme()
    } else {
        shadcn_light_theme()
    }
}
```

**Tokens ported:** `background`, `foreground`, `primary`, `primary-foreground`, `secondary`, `card`, `muted`, `accent`, `destructive`, `border`, `input`, `ring`, sidebar tokens (`sidebar`, `sidebar-foreground`, `sidebar-primary`, `sidebar-accent`, `sidebar-border`, `sidebar-ring`), and radius scale (`sm` through `4xl`).

**Base color:** `neutral` (shadcn default).

### Component Pattern

Components are **builder functions + style modules**, not encapsulated state:

```rust
// src/ui/button.rs — copied into user's project
pub fn button<'a, Message: Clone + 'a>(
    label: impl text::IntoFragment<'a>,
) -> Button<'a, Message, Theme, Renderer> { ... }

pub fn button_style(
    variant: ButtonVariant,
) -> impl Fn(&Theme, button::Status) -> button::Style { ... }
```

- Simple components: style closures on iced built-ins (`button`, `text_input`, `checkbox`, `container`)
- Complex components (`sidebar`, `scroll-area`, `table`): composition of iced 0.14 built-ins; custom `Widget` impl only when built-ins are insufficient
- iced 0.14 `table` widget used for the `table` component

## CLI Design

### Crate Dependencies

- `clap` — CLI parsing
- `reqwest` — fetch templates from GitHub (blocking client for simplicity in v0.1)
- `serde` / `serde_json` — registry parsing
- `toml` — read/write `iced-shadcn.toml`

### Commands (v0.1)

| Command | Purpose |
|---------|---------|
| `iced-shadcn add <component>...` | Add one or more components |
| `iced-shadcn list` | List available components from remote registry |
| `iced-shadcn diff <component>` | Show diff vs installed file without writing |

No `init` command. First `add` creates config and bootstraps shared files.

### `add` Workflow

1. Load or create `iced-shadcn.toml` (prompt for style/path if missing)
2. Fetch `registry.json` from GitHub
3. Resolve component and transitive dependencies (e.g. `card` requires `theme`)
4. If `theme.rs` missing → copy `theme.rs`, `utils.rs`, `mod.rs` scaffold
5. For each component:
   - Fetch `.rs.template` from registry
   - Render template (substitute placeholders)
   - Write to configured `ui_path`
   - Patch `mod.rs` to add `pub mod <name>;` if not present
6. Ensure `Cargo.toml` has iced 0.14 with required features
7. Print summary and usage hint

### Configuration (`iced-shadcn.toml`)

```toml
style = "new-york"
base_color = "neutral"
ui_path = "src/ui"
iced_version = "0.14"
registry_url = "https://github.com/bengidev/iced-shadcn-ui"
```

### Registry Format

```json
{
  "$schema": "./schema.json",
  "name": "iced-shadcn",
  "homepage": "https://github.com/bengidev/iced-shadcn-ui",
  "items": [
    {
      "name": "theme",
      "type": "registry:theme",
      "files": [
        { "path": "styles/new-york/theme.rs.template", "target": "theme.rs" }
      ],
      "dependencies": []
    },
    {
      "name": "button",
      "type": "registry:component",
      "files": [
        { "path": "styles/new-york/button.rs.template", "target": "button.rs" }
      ],
      "registryDependencies": ["theme"],
      "features": ["button"]
    }
  ]
}
```

### Template Rendering

Minimal string substitution (no templating engine):

| Placeholder | Value |
|-------------|-------|
| `{{ui_path}}` | Module path from config (e.g. `crate::ui`) |
| `{{style}}` | `new-york` |
| `{{base_color}}` | `neutral` |

Each template is valid Rust after substitution.

### `mod.rs` Patching

CLI manages a marked section idempotently:

```rust
// iced-shadcn managed start
pub mod theme;
pub mod utils;
pub mod button;
// iced-shadcn managed end
```

User code outside the managed block is never modified.

### Cargo.toml Handling

On first `add`, ensure:

```toml
[dependencies]
iced = { version = "0.14", features = ["advanced", "tokio", "wgpu"] }
```

Feature set is the union of all installed components. CLI merges without overwriting unrelated dependencies.

### Registry Fetch URLs

```
https://raw.githubusercontent.com/bengidev/iced-shadcn-ui/main/registry/registry.json
https://raw.githubusercontent.com/bengidev/iced-shadcn-ui/main/registry/styles/new-york/<component>.rs.template
```

### Error Handling

| Scenario | Behavior |
|----------|----------|
| No network | Clear error; suggest checking `registry_url` |
| Component already exists | Skip with warning; use `diff` to compare |
| Unknown component | Suggest closest match from `list` |
| iced version mismatch | Warn if user's iced version differs from config |

## v0.1 Component Plan

### Component Set (14)

**Foundation (8):** `button`, `input`, `label`, `card`, `badge`, `separator`, `checkbox`, `switch`

**Layout (6):** `sidebar`, `scroll-area`, `table`, `skeleton`, `progress`, `avatar`

**Auto-bootstrapped (not standalone `add` targets):** `theme`, `utils`

### Delivery Order

| Phase | Components | Complexity |
|-------|------------|------------|
| 1 — Theme foundation | `theme`, `utils` | Low |
| 2 — Primitives | `button`, `badge`, `label`, `separator`, `skeleton`, `progress` | Low |
| 3 — Inputs | `input`, `checkbox`, `switch` | Medium |
| 4 — Layout | `card`, `avatar`, `scroll-area` | Medium |
| 5 — Data & nav | `table`, `sidebar` | High |

### Per-Component Scope

Each copied file includes:

- Variants matching shadcn New York (e.g. button: default, destructive, outline, secondary, ghost, link)
- Sizes where applicable (sm, default, lg, icon)
- Style function returning iced `.style()` closure reading from `ShadcnPalette`
- Doc comment with usage example at file top

### Sidebar Scope (v0.1)

Simplified from shadcn's full sidebar:

- Collapsible rail + expanded panel
- Nav item list with active/hover states
- Sidebar-specific design tokens
- Desktop-first; no mobile sheet in v0.1

## Documentation

| Document | Purpose |
|----------|---------|
| `README.md` | Overview, quick start, badges |
| `docs/installation.md` | `cargo install iced-shadcn`, first `add`, config |
| `docs/theming.md` | Token reference, dark/light toggle, palette customization |
| `docs/components/*.md` | One page per component |
| `docs/porting-guide.md` | shadcn → iced translation guide for contributors |
| `docs/contributing.md` | Adding components to the registry |
| `CHANGELOG.md` | Per-release component additions |

## Showcase Example App

`examples/showcase/` — runnable iced app with:

- Sidebar navigation between component categories
- Dark/light toggle in header
- Pages for buttons, inputs, layout, and data components
- Serves as visual QA and live documentation reference

## Testing Strategy (v0.1)

| Layer | Method |
|-------|--------|
| CLI unit tests | Registry parsing, mod.rs patching, toml generation |
| CLI integration | `add button` in temp dir → verify files compile |
| Component compile | Showcase app builds all components |
| Visual QA | Manual via showcase app |

## crates.io Publishing

```toml
[package]
name = "iced-shadcn"
version = "0.1.0"
description = "Add shadcn/ui-inspired components to your iced project"
license = "MIT"
repository = "https://github.com/bengidev/iced-shadcn-ui"
keywords = ["iced", "gui", "shadcn", "ui", "widgets"]
categories = ["gui"]
```

Only the CLI crate is published. Templates remain on GitHub.

## Success Criteria (v0.1)

1. `cargo install iced-shadcn` works from crates.io
2. `iced-shadcn add button` in a fresh iced 0.14 app copies files and compiles
3. All 14 components available via `add`
4. Dark/light toggle works in showcase app
5. Documentation covers install, theming, and each component
6. GitHub repo `bengidev/iced-shadcn-ui` is public with registry fetchable by CLI

## Future Work (post-v0.1)

- Overlay infrastructure + components (`dialog`, `popover`, `dropdown-menu`, `tooltip`, `select`, `tabs`)
- `--ref` flag to pin registry git tags
- Additional styles beyond New York
- mdBook documentation site
- Visual regression tests
