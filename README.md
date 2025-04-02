## 🧩 Tauri Plugin: Native Context Menu for Tauri v2

A plugin for displaying native context menus in [Tauri](https://tauri.app) — now updated for **Tauri v2.x** compatibility.

> **✨ Maintained Fork with Tauri 2 Support**
>
> This fork extends the original [tauri-plugin-context-menu](https://github.com/c2r0b/tauri-plugin-context-menu) to work with Tauri 2.0+ and provides optional custom menu initialization.

---

### ✅ OS Support

| Windows | macOS | Linux |
|---------|--------|-------|
| ✅       | ✅      | ✅     |

---

### 🔧 Installation

Add to your `Cargo.toml` (using this fork):

```toml
[dependencies]
tauri-plugin-context-menu = { git = "https://github.com/YOUR_GITHUB_USERNAME/tauri-plugin-context-menu", branch = "main" }
```

Then, initialize in your Tauri app:

```rust
use tauri_plugin_context_menu::{init_with_menu, MenuItem};

let custom_menu = vec![
    MenuItem::new("Copy"),
    MenuItem::new("Paste"),
];

tauri::Builder::default()
    .plugin(init_with_menu(custom_menu)) // or just `.plugin(init())`
    .run(tauri::generate_context!())
    .expect("failed to run app");
```

---

### 📦 TypeScript / JavaScript Utility

Install the helper package:

```bash
npm i tauri-plugin-context-menu
```

Use it in your frontend:

```ts
import { showMenu } from "tauri-plugin-context-menu";

showMenu({
  pos: { x: 100, y: 100 },
  items: [
    {
      label: "Hello",
      event: () => console.log("clicked!"),
    },
  ],
});
```

---

### ✨ Features

- Nested submenus
- Icons
- Keyboard shortcuts
- Separators
- Custom payloads
- Per-platform styling
- Optional menu themes (`light`, `dark`)
- Positioning via screen coordinates or cursor
- Supports `s3://` or custom protocols in Tauri 2

---

### 🧪 Examples

- `examples/vanilla` — plain JS
- `examples/ts-utility` — with the TS helper

To run:

```bash
npm run examples/vanilla
```

---

### 📚 API Overview

#### `MenuItem`

| Option        | Type              | Optional | Description                                |
|---------------|-------------------|----------|--------------------------------------------|
| `label`       | `string`          | ✔️       | Displayed text                             |
| `event`       | `string` or `fn`  | ✔️       | Event to emit or callback to call          |
| `disabled`    | `boolean`         | ✔️       | If true, item is disabled                  |
| `checked`     | `boolean`         | ✔️       | Checked state (e.g. for toggles)           |
| `subitems`    | `MenuItem[]`      | ✔️       | Nested submenu                             |
| `shortcut`    | `string`          | ✔️       | Shown on the right                         |
| `icon`        | `MenuItemIcon`    | ✔️       | Icon for the item                          |
| `is_separator`| `boolean`         | ✔️       | Whether this is a divider line             |
| `payload`     | `string`          | ✔️       | Data passed to the event                   |

#### `MenuItemIcon`

| Option  | Type     | Optional | Default | Description            |
|---------|----------|----------|---------|------------------------|
| `path`  | `string` | ❌       |         | Absolute path to image |
| `width` | `number` | ✔️       | `16`    | Icon width in pixels   |
| `height`| `number` | ✔️       | `16`    | Icon height in pixels  |

#### `Position`

| Option       | Type    | Optional | Default | Description                                       |
|--------------|---------|----------|---------|--------------------------------------------------|
| `x`, `y`     | `number`| ❌       |         | Coordinates                                      |
| `is_absolute`| `bool`  | ✔️       | `false` | Whether relative to screen (vs window)           |

---

### 🎧 Events

#### Menu Item Click

```ts
listen("my-event", (e) => {
  console.log("clicked", e.payload);
});
```

#### Menu Did Close

```ts
listen("menu-did-close", () => {
  console.log("menu closed");
});
```

---

### 🛠 Maintenance Notes

This fork is actively maintained for Tauri v2 compatibility, including:

- API parity with Tauri 2 event system
- Optional default menu injection (`init_with_menu`)
- Compatibility with custom protocols like `s3://`

---

Let me know your GitHub username and I’ll replace the repo URL above with your fork!
