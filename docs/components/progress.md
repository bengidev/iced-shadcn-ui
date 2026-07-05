# Progress

Horizontal progress bar for completion indicators.

## Install

```bash
iced-shadcn add progress
```

## Usage

```rust
use ui::progress::progress;

progress(62.0)  // value from 0.0 to 100.0
```

## Example

```rust
column![
    text("Uploading…"),
    progress(state.upload_percent),
]
.spacing(8)
```

Track uses `secondary`; fill uses `primary` from the active palette.
