# node-editor-cpp

A small [Slint](https://slint.dev) node / dataflow editor in C++, modeled on
[Carmenta Studio](https://docs.carmenta.com/pages/studio.html): a horizontal
forest (root at left), state-coloured class nodes, and light-blue bezier
connector wires.

![screenshot](docs/screenshot.png)

It exists to show that an interactive node editor is a build-it-yourself
component in Slint, not a fight-the-framework one. The whole thing is one
`.slint` UI file (~430 lines) plus ~120 lines of C++ that only owns the models.

## Features

- Bezier wires whose control points are **bound** to pin positions — no path
  strings, no per-frame redraw code (`node-editor.slint`, the `Path`/`CubicTo`)
- Pan (drag empty canvas) and zoom-to-cursor (scroll wheel)
- Drag nodes with wires following live
- Click an output pin, then an input pin, to connect
- Selection + a live property editor (rename, validation state, delete)
- Class palette to add nodes

## Build & run

Needs CMake ≥ 3.21, a C++20 compiler, and — for the first configure, which
fetches and builds Slint — a [Rust toolchain](https://rustup.rs).

```sh
cmake -B build -S .
cmake --build build
./build/node-editor        # build/Debug/node-editor.exe on Windows
```

To use a pre-installed Slint C++ package instead of fetching it, point CMake at
it with `-DCMAKE_PREFIX_PATH=/path/to/slint`.

## Layout

| File | |
|---|---|
| `node-editor.slint` | the UI — all drawing and interaction |
| `main.cpp` | backend — models (`VectorModel`) and callbacks |
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
