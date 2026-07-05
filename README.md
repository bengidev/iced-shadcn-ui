# iced-shadcn-ui

shadcn/ui-inspired components for [iced](https://github.com/iced-rs/iced) 0.14.

Copy self-contained Rust source into your project with the `iced-shadcn` CLI — no runtime widget crate dependency. You own the code.

## Quick start

```bash
cargo install iced-shadcn
cd your-iced-app
iced-shadcn add button
```

The first `add` creates `iced-shadcn.toml`, copies `theme.rs` and `utils.rs`, patches `src/ui/mod.rs`, and updates `Cargo.toml` with iced 0.14.

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

```bash
iced-shadcn list          # show all available components
iced-shadcn add card badge
```

## Showcase

Run the example app to preview every component with dark/light theming:

```bash
cargo run -p showcase
```

## Documentation

- [Installation](docs/installation.md)
- [Theming](docs/theming.md)
- [Porting guide](docs/porting-guide.md) — translate shadcn/ui to iced
- [Contributing](docs/contributing.md)
- [Component reference](docs/components/)

## License

MIT — see [LICENSE](LICENSE).
