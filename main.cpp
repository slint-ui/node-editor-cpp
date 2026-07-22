// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

// Carmenta-style dataflow node editor — C++ backend.
// Owns the models and mutates them on callbacks; all interaction is in
// node-editor.slint.

#include "node-editor.h"

#include <memory>
#include <vector>

using slint::SharedString;
using slint::VectorModel;

static NodeData make_node(const char *title, const char *subtitle, float x, float y, int state)
{
    // `length` in .slint maps to `float` (logical pixels) in generated C++.
    return NodeData{ .title = title, .subtitle = subtitle, .x = x, .y = y, .state = state };
}

int main()
{
    auto ui = NodeEditor::create();

    // ---- initial Carmenta-like map-config forest (root at left) ----
    auto nodes = std::make_shared<VectorModel<NodeData>>(std::vector<NodeData>{
        make_node("View", "MainView", 30.f, 300.f, 0),
        make_node("GeometryLayer", "Roads", 300.f, 150.f, 0),
        make_node("GeometryLayer", "Buildings", 300.f, 300.f, 0),
        make_node("TextLayer", "Labels", 300.f, 450.f, 1),
        make_node("FileDataSet", "roads.shp", 570.f, 150.f, 0),
        make_node("MemoryDataSet", "buildings", 570.f, 300.f, 0),
        make_node("WmsDataSet", "basemap", 570.f, 450.f, 2),
        make_node("StyleOperator", "RoadStyle", 840.f, 150.f, 0),
    });
    auto wires = std::make_shared<VectorModel<WireData>>(std::vector<WireData>{
        WireData{ .from = 0, .to = 1 }, WireData{ .from = 0, .to = 2 },
        WireData{ .from = 0, .to = 3 }, WireData{ .from = 1, .to = 4 },
        WireData{ .from = 2, .to = 5 }, WireData{ .from = 3, .to = 6 },
        WireData{ .from = 4, .to = 7 },
    });
    ui->set_nodes(nodes);
    ui->set_wires(wires);

    auto classes = std::make_shared<VectorModel<SharedString>>(std::vector<SharedString>{
        "View", "GeometryLayer", "TextLayer", "RasterLayer", "FileDataSet",
        "MemoryDataSet", "WmsDataSet", "StyleOperator", "FilterOperator", "ProjectionOperator",
    });
    ui->set_classes(classes);

    slint::ComponentWeakHandle<NodeEditor> weak(ui);

    // ---- add node (from palette click) ----
    ui->on_add_node([nodes, weak](const SharedString &cls) {
        int n = static_cast<int>(nodes->row_count());
        float x = 120.f + (n % 4) * 24.f; // stagger so new nodes don't stack
        float y = 40.f + (n % 6) * 20.f;
        nodes->push_back(make_node(cls.data(), "", x, y, 1)); // new nodes start "incomplete"
        if (auto ui = weak.lock())
            (*ui)->set_selected(static_cast<int>(nodes->row_count()) - 1);
    });

    // ---- move node (live drag) ----
    ui->on_node_moved([nodes](int i, float x, float y) {
        if (auto d = nodes->row_data(i)) {
            d->x = x;
            d->y = y;
            nodes->set_row_data(i, *d);
        }
    });

    // ---- rename ----
    ui->on_rename_node([nodes](int i, const SharedString &name) {
        if (auto d = nodes->row_data(i)) {
            d->subtitle = name;
            nodes->set_row_data(i, *d);
        }
    });

    // ---- set validation state ----
    ui->on_set_node_state([nodes](int i, int s) {
        if (auto d = nodes->row_data(i)) {
            d->state = s;
            nodes->set_row_data(i, *d);
        }
    });

    // ---- add wire (from click-to-connect) ----
    ui->on_add_wire([wires](int from, int to) {
        if (from == to)
            return;
        for (size_t k = 0; k < wires->row_count(); ++k) {
            auto w = wires->row_data(k);
            if (w && w->from == from && w->to == to)
                return; // already exists
        }
        wires->push_back(WireData{ .from = from, .to = to });
    });

    // ---- delete node (and reindex wires) ----
    ui->on_delete_node([nodes, wires](int idx) {
        if (idx < 0 || static_cast<size_t>(idx) >= nodes->row_count())
            return;
        nodes->erase(idx);
        // drop wires touching idx, shift the rest down
        std::vector<WireData> kept;
        for (size_t k = 0; k < wires->row_count(); ++k) {
            auto w = wires->row_data(k);
            if (!w || w->from == idx || w->to == idx)
                continue;
            kept.push_back(WireData{
                .from = w->from > idx ? w->from - 1 : w->from,
                .to = w->to > idx ? w->to - 1 : w->to,
            });
        }
        wires->set_vector(std::move(kept));
    });

    ui->run();
    return 0;
}
