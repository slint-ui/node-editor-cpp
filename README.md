# node-editor-cpp

A small [Slint](https://slint.dev) node / dataflow editor in C++, modeled on
[Carmenta Studio](https://docs.carmenta.com/pages/studio.html): a horizontal
forest (root at left), state-coloured class nodes, and light-blue bezier
connector wires.

**[▶ Live browser demo](https://slint-ui.github.io/node-editor-cpp/)** · **[⬇ Prebuilt C++ binaries](https://github.com/slint-ui/node-editor-cpp/releases/latest)** (macOS / Linux / Windows)

![screenshot](docs/screenshot.png)

It exists to show that an interactive node editor is a build-it-yourself
component in Slint, not a fight-the-framework one. The whole thing is one
`.slint` UI file (~430 lines) plus ~120 lines of backend that only owns the
models — provided in both **C++** (`main.cpp`) and **Rust** (`main.rs`). The
same UI also compiles to **WebAssembly** for a browser demo.

## Features

- Bezier wires whose control points are **bound** to pin positions — no path
  strings, no per-frame redraw code (`node-editor.slint`, the `Path`/`CubicTo`)
- Pan (drag empty canvas) and zoom-to-cursor (scroll wheel)
- Drag nodes with wires following live
- Click an output pin, then an input pin, to connect
- Selection + a live property editor (rename, validation state, delete)
- Class palette to add nodes
- Live **CPU% + FPS counter** in the toolbar (native shows real process CPU;
  the browser can't, so it reads "—")

### About the counter

Slint only repaints when something changes, so when you're not interacting the
editor renders **zero frames** and uses **~0% CPU** — even while dragging a wire,
CPU stays in the low single digits. Note the counter reads ~2 FPS at rest rather
than 0: refreshing the on-screen number twice a second is itself a change, so it
triggers ~2 repaints/second — the counter measuring itself. Without it on screen,
idle is a true 0.

## Prebuilt binaries

Grab one for your OS from the [latest release](https://github.com/slint-ui/node-editor-cpp/releases/latest).
They're unsigned, so on first launch:

- **macOS** — right-click → **Open** → **Open**, or
  `xattr -d com.apple.quarantine node-editor-macos-arm64 && chmod +x node-editor-macos-arm64`
- **Windows** — SmartScreen may warn: **More info → Run anyway**
- **Linux** — `chmod +x node-editor-linux-x64 && ./node-editor-linux-x64`

## Build & run — C++

Needs CMake ≥ 3.21, a C++20 compiler, and — for the first configure, which
fetches and builds Slint — a [Rust toolchain](https://rustup.rs).

```sh
cmake -B build -S .
cmake --build build
./build/node-editor        # build/Debug/node-editor.exe on Windows
```

To use a pre-installed Slint C++ package instead of fetching it, point CMake at
it with `-DCMAKE_PREFIX_PATH=/path/to/slint`.

## Build & run — Rust

```sh
cargo run
```

## Build & run — WebAssembly (browser demo)

Slint has no C++→WASM path, so the browser build uses the Rust backend — the
UI and behaviour are identical.

```sh
wasm-pack build --release --target web   # -> ./pkg
python3 -m http.server 8777              # serve this folder
# open http://localhost:8777/index.html
```

`index.html` + `pkg/` is a fully static bundle; drop it on any static host.

## Layout

| File | |
|---|---|
| `node-editor.slint` | the UI — all drawing and interaction (shared) |
| `main.cpp` | C++ backend — `slint::VectorModel` + callbacks |
| `main.rs` | Rust backend — `VecModel` + callbacks (also the WASM entry) |
| `CMakeLists.txt` / `Cargo.toml` | C++ / Rust build |
| `index.html` | WASM host page |
| `preview.slint` | static wrapper for `slint-viewer --screenshot` (no backend) |

## How it maps to Slint

| Node-editor need | Slint primitive |
|---|---|
| Wires / curves | `Path` + `MoveTo`/`CubicTo`, control points as bound properties |
| Pan | one background `TouchArea` |
| Zoom | a single `zoom` factor applied at the canvas level |
| Drag | per-node `TouchArea` + `absolute-position` |
| Dynamic add/remove | host-owned `slint::VectorModel<T>` |

## License

MIT — see [LICENSE](LICENSE).
