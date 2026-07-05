# Contributing

Thank you for helping improve iced-shadcn-ui. This guide covers adding or updating components in the registry.

## Repository layout

```
iced-shadcn-ui/
├── crates/iced-shadcn-ui/  # CLI (published to crates.io as iced-shadcn-ui)
├── registry/
│   ├── registry.json       # component manifest
│   └── styles/new-york/    # *.rs.template files
├── examples/showcase/      # visual QA app
└── docs/                   # user-facing documentation
```

## Adding a component

### 1. Create the template

Add `registry/styles/new-york/<name>.rs.template`. Use `{{ui_path}}` for import paths:

```rust
use crate::{{ui_path}}::theme::palette;
```

Follow existing components: factory function, variant enums, `.style()` with `palette(theme)`.

### 2. Register in `registry.json`

Add an entry to `items`:

```json
{
  "name": "my-component",
  "type": "registry:component",
  "files": [
    { "path": "styles/new-york/my-component.rs.template", "target": "my_component.rs" }
  ],
  "registryDependencies": ["theme", "utils"],
  "features": []
}
```

- `name` — CLI name (kebab-case)
- `target` — output filename (snake_case)
- `registryDependencies` — always include `theme` and `utils` for styled components

### 3. Update the showcase

Copy the rendered component into `examples/showcase/src/ui/` (or run `iced-shadcn add` in the showcase directory) and add a demo to the appropriate page under `examples/showcase/src/pages/`.

### 4. Write documentation

Add `docs/components/<name>.md` with a usage example.

### 5. Run tests

```bash
cargo test -p iced-shadcn-ui
cargo run -p showcase
```

CLI tests cover registry parsing, dependency resolution, `mod.rs` patching, and local `add` in a temp directory.

## Updating an existing component

1. Edit the `.rs.template` file
2. Update the matching file in `examples/showcase/src/ui/` for visual QA
3. Update `docs/components/<name>.md` if the API changed
4. Users who already copied the file can run `iced-shadcn diff <name>` to preview upstream changes

## CLI development

```bash
cargo build -p iced-shadcn-ui
cargo test -p iced-shadcn-ui
```

Integration tests in `crates/iced-shadcn-ui/tests/` exercise `add` and `list` against the local registry.

## Pull requests

- Keep components self-contained (no shared runtime crate)
- Match shadcn New York styling where practical
- Include showcase coverage for new components
- Update `CHANGELOG.md` under `[Unreleased]` for user-visible changes

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
