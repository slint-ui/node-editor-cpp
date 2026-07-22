// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// Carmenta-Studio-style dataflow node editor — Slint PoC.
// The Rust side owns the models and mutates them; every gesture
// (drag / pan / zoom / select / link) is handled in the .slint UI.

use std::rc::Rc;

use slint::{Model, ModelRc, SharedString, VecModel};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

slint::include_modules!();

fn node(title: &str, subtitle: &str, x: f32, y: f32, state: i32) -> NodeData {
    // `length` in .slint maps to f32 (logical pixels) in generated Rust.
    NodeData { title: title.into(), subtitle: subtitle.into(), x, y, state }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
fn main() -> Result<(), slint::PlatformError> {
    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    console_error_panic_hook::set_once();

    let ui = NodeEditor::new()?;

    // ---- initial Carmenta-like map-config forest (root at left) ----
    let nodes: Vec<NodeData> = vec![
        node("View", "MainView", 30.0, 300.0, 0),
        node("GeometryLayer", "Roads", 300.0, 150.0, 0),
        node("GeometryLayer", "Buildings", 300.0, 300.0, 0),
        node("TextLayer", "Labels", 300.0, 450.0, 1),
        node("FileDataSet", "roads.shp", 570.0, 150.0, 0),
        node("MemoryDataSet", "buildings", 570.0, 300.0, 0),
        node("WmsDataSet", "basemap", 570.0, 450.0, 2),
        node("StyleOperator", "RoadStyle", 840.0, 150.0, 0),
    ];
    let wires: Vec<WireData> = vec![
        WireData { from: 0, to: 1 },
        WireData { from: 0, to: 2 },
        WireData { from: 0, to: 3 },
        WireData { from: 1, to: 4 },
        WireData { from: 2, to: 5 },
        WireData { from: 3, to: 6 },
        WireData { from: 4, to: 7 },
    ];

    let nodes = Rc::new(VecModel::from(nodes));
    let wires = Rc::new(VecModel::from(wires));
    ui.set_nodes(ModelRc::from(nodes.clone()));
    ui.set_wires(ModelRc::from(wires.clone()));

    let classes: Vec<SharedString> = [
        "View",
        "GeometryLayer",
        "TextLayer",
        "RasterLayer",
        "FileDataSet",
        "MemoryDataSet",
        "WmsDataSet",
        "StyleOperator",
        "FilterOperator",
        "ProjectionOperator",
    ]
    .iter()
    .map(|s| (*s).into())
    .collect();
    ui.set_classes(ModelRc::from(Rc::new(VecModel::from(classes))));

    // ---- add node (from palette click) ----
    {
        let nodes = nodes.clone();
        let ui_weak = ui.as_weak();
        ui.on_add_node(move |class| {
            let n = nodes.row_count();
            // stagger new nodes so they don't stack exactly
            let x = 120.0 + (n as f32 % 4.0) * 24.0;
            let y = 40.0 + (n as f32 % 6.0) * 20.0;
            nodes.push(node(class.as_str(), "", x, y, 1)); // new nodes start "incomplete"
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_selected(nodes.row_count() as i32 - 1);
            }
        });
    }

    // ---- move node (live drag) ----
    {
        let nodes = nodes.clone();
        ui.on_node_moved(move |i, x, y| {
            if let Some(mut d) = nodes.row_data(i as usize) {
                d.x = x;
                d.y = y;
                nodes.set_row_data(i as usize, d);
            }
        });
    }

    // ---- rename ----
    {
        let nodes = nodes.clone();
        ui.on_rename_node(move |i, name| {
            if let Some(mut d) = nodes.row_data(i as usize) {
                d.subtitle = name;
                nodes.set_row_data(i as usize, d);
            }
        });
    }

    // ---- set validation state ----
    {
        let nodes = nodes.clone();
        ui.on_set_node_state(move |i, s| {
            if let Some(mut d) = nodes.row_data(i as usize) {
                d.state = s;
                nodes.set_row_data(i as usize, d);
            }
        });
    }

    // ---- add wire (from click-to-connect) ----
    {
        let wires = wires.clone();
        ui.on_add_wire(move |from, to| {
            if from == to {
                return;
            }
            let exists = wires
                .iter()
                .any(|w| w.from == from && w.to == to);
            if !exists {
                wires.push(WireData { from, to });
            }
        });
    }

    // ---- delete node (and reindex wires) ----
    {
        let nodes = nodes.clone();
        let wires = wires.clone();
        ui.on_delete_node(move |idx| {
            let idx = idx as i32;
            if idx < 0 || idx as usize >= nodes.row_count() {
                return;
            }
            nodes.remove(idx as usize);
            // drop wires touching idx, shift the rest down
            let kept: Vec<WireData> = wires
                .iter()
                .filter(|w| w.from != idx && w.to != idx)
                .map(|w| WireData {
                    from: if w.from > idx { w.from - 1 } else { w.from },
                    to: if w.to > idx { w.to - 1 } else { w.to },
                })
                .collect();
            wires.set_vec(kept);
        });
    }

    ui.run()
}
