//! TD Workshop — Phase A native desktop shell (egui/eframe).
//!
//! Focus: **Map Studio**. Tower Bench / Enemy Lab / Playtest are stubbed.
//!
//! ## Rebuild path behavior
//! "Rebuild path" runs an orthogonal BFS from `spawn` across tiles whose kind is
//! `Path`, preferring a route that ends at `leak` when reachable. If leak is not
//! reachable, the longest BFS discovery order toward the farthest reachable Path
//! cell is used (still orthogonal 4-neighbor only). The resulting waypoint list
//! is written to `MapDef::path`. Spawn/leak are not moved by rebuild.

use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2 as EVec2};
use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;
use td_core::{
    Cell, MapDef, PathValidationError, Tile, TileKind, HEIGHT_MAX, HEIGHT_MIN,
};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("TD Workshop — Map Studio"),
        ..Default::default()
    };
    eframe::run_native(
        "TD Workshop",
        options,
        Box::new(|_cc| Box::new(WorkshopApp::new())),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tool {
    Path,
    Placeable,
    Blocked,
    Raise,
    Lower,
    Spawn,
    Leak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Panel {
    MapStudio,
    TowerBench,
    EnemyLab,
    Playtest,
}

struct WorkshopApp {
    panel: Panel,
    map: MapDef,
    tool: Tool,
    /// Pending new-map dimensions (applied on "Create").
    new_w: u32,
    new_d: u32,
    status: String,
    path_error: Option<String>,
    last_paint_cell: Option<Cell>,
    painting: bool,
    file_path: PathBuf,
    /// Optional waypoint list editor buffer (comma-separated "x,y").
    waypoint_edit: String,
    show_height_labels: bool,
}

impl WorkshopApp {
    fn new() -> Self {
        let map = blank_map(16, 12);
        let mut app = Self {
            panel: Panel::MapStudio,
            map,
            tool: Tool::Path,
            new_w: 16,
            new_d: 12,
            status: "New 16×12 map — paint Path, set Spawn/Leak, Rebuild path.".into(),
            path_error: None,
            last_paint_cell: None,
            painting: false,
            file_path: PathBuf::from("packs/dev_map.json"),
            waypoint_edit: String::new(),
            show_height_labels: true,
        };
        app.refresh_validation();
        app.sync_waypoint_edit();
        app
    }

    fn refresh_validation(&mut self) {
        self.path_error = match self.map.validate_path() {
            Ok(()) => None,
            Err(e) => Some(format_path_error(&e)),
        };
    }

    fn sync_waypoint_edit(&mut self) {
        self.waypoint_edit = self
            .map
            .path
            .iter()
            .map(|c| format!("{},{}", c.x, c.y))
            .collect::<Vec<_>>()
            .join(" → ");
    }

    fn apply_brush(&mut self, cell: Cell) {
        if !self.map.in_bounds(cell) {
            return;
        }
        match self.tool {
            Tool::Path => {
                let h = self.map.get_tile(cell).map(|t| t.height).unwrap_or(0);
                self.map.set_tile(cell, Tile::path(h));
            }
            Tool::Placeable => {
                let h = self.map.get_tile(cell).map(|t| t.height).unwrap_or(0);
                self.map.set_tile(cell, Tile::placeable(h));
            }
            Tool::Blocked => {
                let h = self.map.get_tile(cell).map(|t| t.height).unwrap_or(0);
                self.map.set_tile(cell, Tile::blocked(h));
            }
            Tool::Raise => {
                self.map.raise(cell);
            }
            Tool::Lower => {
                self.map.lower(cell);
            }
            Tool::Spawn => {
                if matches!(
                    self.map.get_tile(cell).map(|t| t.kind),
                    Some(TileKind::Path)
                ) {
                    self.map.spawn = cell;
                    self.status = format!("Spawn set to ({}, {})", cell.x, cell.y);
                } else {
                    self.status = "Spawn must be on a Path cell.".into();
                }
            }
            Tool::Leak => {
                if matches!(
                    self.map.get_tile(cell).map(|t| t.kind),
                    Some(TileKind::Path)
                ) {
                    self.map.leak = cell;
                    self.status = format!("Leak set to ({}, {})", cell.x, cell.y);
                } else {
                    self.status = "Leak must be on a Path cell.".into();
                }
            }
        }
        self.refresh_validation();
        self.sync_waypoint_edit();
    }

    fn rebuild_path(&mut self) {
        match rebuild_path_from_spawn(&self.map) {
            Ok(path) => {
                let n = path.len();
                self.map.path = path;
                self.status = format!(
                    "Rebuilt path: {n} waypoints (orthogonal BFS from spawn across Path tiles)."
                );
            }
            Err(msg) => {
                self.status = msg;
            }
        }
        self.refresh_validation();
        self.sync_waypoint_edit();
    }

    fn save_map(&mut self) {
        match save_map_json(&self.map, &self.file_path) {
            Ok(()) => {
                self.status = format!("Saved {}", self.file_path.display());
            }
            Err(e) => {
                self.status = format!("Save failed: {e}");
            }
        }
    }

    fn load_map(&mut self) {
        match load_map_json(&self.file_path) {
            Ok(map) => {
                self.map = map;
                self.status = format!("Loaded {}", self.file_path.display());
                self.refresh_validation();
                self.sync_waypoint_edit();
            }
            Err(e) => {
                self.status = format!("Load failed: {e}");
            }
        }
    }

    fn new_map(&mut self) {
        let w = self.new_w.clamp(2, 64);
        let d = self.new_d.clamp(2, 64);
        self.new_w = w;
        self.new_d = d;
        self.map = blank_map(w, d);
        self.status = format!("Created blank {w}×{d} map (all Blocked, height 0).");
        self.refresh_validation();
        self.sync_waypoint_edit();
    }

    fn load_stub(&mut self) {
        self.map = MapDef::stub();
        self.new_w = self.map.width;
        self.new_d = self.map.depth;
        self.status = "Loaded MapDef::stub() sample.".into();
        self.refresh_validation();
        self.sync_waypoint_edit();
    }
}

impl eframe::App for WorkshopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New map…").clicked() {
                        self.new_map();
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        self.save_map();
                        ui.close_menu();
                    }
                    if ui.button("Load").clicked() {
                        self.load_map();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Load stub sample").clicked() {
                        self.load_stub();
                        ui.close_menu();
                    }
                });
                ui.separator();
                ui.selectable_value(&mut self.panel, Panel::MapStudio, "Map Studio");
                ui.selectable_value(&mut self.panel, Panel::TowerBench, "Tower Bench");
                ui.selectable_value(&mut self.panel, Panel::EnemyLab, "Enemy Lab");
                ui.selectable_value(&mut self.panel, Panel::Playtest, "Playtest");
            });
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.monospace(self.file_path.display().to_string());
                });
            });
            if let Some(err) = &self.path_error {
                ui.colored_label(Color32::from_rgb(220, 80, 80), format!("Path invalid: {err}"));
            } else {
                ui.colored_label(Color32::from_rgb(80, 180, 100), "Path valid ✓");
            }
        });

        match self.panel {
            Panel::MapStudio => self.ui_map_studio(ctx),
            Panel::TowerBench => coming_soon(ctx, "Tower Bench"),
            Panel::EnemyLab => coming_soon(ctx, "Enemy Lab"),
            Panel::Playtest => coming_soon(ctx, "Playtest"),
        }
    }
}

impl WorkshopApp {
    fn ui_map_studio(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("tools")
            .default_width(220.0)
            .show(ctx, |ui| {
                ui.heading("Map Studio");
                ui.separator();

                ui.label("New map size");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut self.new_w).clamp_range(2..=64).prefix("W "));
                    ui.add(egui::DragValue::new(&mut self.new_d).clamp_range(2..=64).prefix("D "));
                    if ui.button("Create").clicked() {
                        self.new_map();
                    }
                });
                ui.label(format!(
                    "Current: {}×{}  height clamp [{}, {}]",
                    self.map.width, self.map.depth, HEIGHT_MIN, HEIGHT_MAX
                ));

                ui.separator();
                ui.label("Brush");
                for (t, label) in [
                    (Tool::Path, "Path"),
                    (Tool::Placeable, "Placeable"),
                    (Tool::Blocked, "Blocked"),
                    (Tool::Raise, "Raise (+1)"),
                    (Tool::Lower, "Lower (−1)"),
                    (Tool::Spawn, "Set Spawn"),
                    (Tool::Leak, "Set Leak"),
                ] {
                    ui.selectable_value(&mut self.tool, t, label);
                }

                ui.separator();
                ui.checkbox(&mut self.show_height_labels, "Height labels");

                ui.separator();
                ui.label("Path");
                ui.label(format!(
                    "Spawn ({}, {})  →  Leak ({}, {})",
                    self.map.spawn.x, self.map.spawn.y, self.map.leak.x, self.map.leak.y
                ));
                ui.label(format!("{} waypoints", self.map.path.len()));
                ui.add(
                    egui::TextEdit::multiline(&mut self.waypoint_edit)
                        .desired_rows(3)
                        .font(egui::TextStyle::Monospace),
                );
                if ui
                    .button("Rebuild path")
                    .on_hover_text(
                        "Orthogonal BFS from spawn over Path tiles; prefers route ending at leak.",
                    )
                    .clicked()
                {
                    self.rebuild_path();
                }
                if ui.button("Apply waypoint text").clicked() {
                    match parse_waypoints(&self.waypoint_edit) {
                        Ok(path) => {
                            self.map.path = path;
                            self.status = "Applied waypoint list from text.".into();
                            self.refresh_validation();
                            self.sync_waypoint_edit();
                        }
                        Err(e) => self.status = e,
                    }
                }

                ui.separator();
                ui.label("I/O (edit path, then Save/Load)");
                let mut path_str = self.file_path.display().to_string();
                if ui.text_edit_singleline(&mut path_str).changed() {
                    self.file_path = PathBuf::from(path_str);
                }
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        self.save_map();
                    }
                    if ui.button("Load").clicked() {
                        self.load_map();
                    }
                });
                if ui.button("Use packs/dev_map.json").clicked() {
                    self.file_path = PathBuf::from("packs/dev_map.json");
                }
                if ui.button("Load stub sample").clicked() {
                    self.load_stub();
                }

                ui.separator();
                ui.collapsing("Legend", |ui| {
                    legend_swatch(ui, Color32::from_rgb(70, 130, 200), "Path");
                    legend_swatch(ui, Color32::from_rgb(90, 160, 90), "Placeable");
                    legend_swatch(ui, Color32::from_rgb(60, 60, 70), "Blocked");
                    ui.label("Tint: darker = lower, lighter = higher");
                    ui.label("S / L markers = spawn / leak");
                });
            });

        egui::SidePanel::right("preview")
            .default_width(320.0)
            .show(ctx, |ui| {
                ui.heading("Extruded preview");
                ui.label("Stacked boxes by height (fake isometric).");
                ui.separator();
                draw_extruded_preview(ui, &self.map);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Top-down grid");
            ui.label("Click / drag to paint with the current brush.");
            draw_topdown_grid(ui, self);
        });
    }
}

fn coming_soon(ctx: &egui::Context, name: &str) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(120.0);
            ui.heading(format!("{name} — Coming soon"));
            ui.label("Phase A focuses on Map Studio. This panel is a stub.");
        });
    });
}

fn legend_swatch(ui: &mut egui::Ui, color: Color32, label: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(EVec2::splat(14.0), Sense::hover());
        ui.painter().rect_filled(rect, 2.0, color);
        ui.label(label);
    });
}

fn blank_map(width: u32, depth: u32) -> MapDef {
    let tiles = vec![Tile::blocked(0); (width * depth) as usize];
    MapDef {
        width,
        depth,
        tiles,
        path: Vec::new(),
        spawn: Cell::new(0, 0),
        leak: Cell::new(0, 0),
    }
}

fn tile_base_color(kind: TileKind) -> Color32 {
    match kind {
        TileKind::Path => Color32::from_rgb(70, 130, 200),
        TileKind::Placeable => Color32::from_rgb(90, 160, 90),
        TileKind::Blocked => Color32::from_rgb(55, 55, 65),
    }
}

fn tint_by_height(base: Color32, height: i8) -> Color32 {
    // height -2..=4 → factor ~0.55..=1.15
    let t = (height as f32 - HEIGHT_MIN as f32) / (HEIGHT_MAX - HEIGHT_MIN) as f32;
    let factor = 0.55 + t * 0.60;
    let scale = |c: u8| ((c as f32 * factor).clamp(0.0, 255.0)) as u8;
    Color32::from_rgb(scale(base.r()), scale(base.g()), scale(base.b()))
}

fn draw_topdown_grid(ui: &mut egui::Ui, app: &mut WorkshopApp) {
    let w = app.map.width as f32;
    let d = app.map.depth as f32;
    let avail = ui.available_size();
    let pad = 8.0;
    let cell = ((avail.x - pad * 2.0) / w)
        .min((avail.y - pad * 2.0) / d)
        .max(8.0)
        .min(48.0);
    let grid_w = cell * w;
    let grid_h = cell * d;
    let (response, painter) =
        ui.allocate_painter(EVec2::new(grid_w + pad * 2.0, grid_h + pad * 2.0), Sense::click_and_drag());
    let origin = response.rect.min + EVec2::new(pad, pad);

    // Paint interaction: one brush apply per cell while the pointer is down.
    // Raise/Lower/Spawn/Leak also use that rule so a drag does not spam ±1.
    if response.drag_started() || response.clicked() {
        app.painting = true;
        app.last_paint_cell = None;
    }
    if response.drag_stopped() || !ui.input(|i| i.pointer.any_down()) {
        app.painting = false;
        app.last_paint_cell = None;
    }
    if app.painting {
        if let Some(pos) = response.interact_pointer_pos() {
            let lx = ((pos.x - origin.x) / cell).floor() as i32;
            let ly = ((pos.y - origin.y) / cell).floor() as i32;
            let cell_pos = Cell::new(lx, ly);
            if app.map.in_bounds(cell_pos) && app.last_paint_cell != Some(cell_pos) {
                app.apply_brush(cell_pos);
                app.last_paint_cell = Some(cell_pos);
            }
        }
    }

    // Draw cells (y increases downward in screen space, matching grid y)
    for y in 0..app.map.depth as i32 {
        for x in 0..app.map.width as i32 {
            let c = Cell::new(x, y);
            let tile = app.map.get_tile(c).unwrap();
            let color = tint_by_height(tile_base_color(tile.kind), tile.height);
            let min = origin + EVec2::new(x as f32 * cell, y as f32 * cell);
            let rect = Rect::from_min_size(min, EVec2::splat(cell - 1.0));
            painter.rect_filled(rect, 2.0, color);

            // Path order index
            if let Some(idx) = app.map.path.iter().position(|&p| p == c) {
                painter.text(
                    rect.left_top() + EVec2::new(2.0, 1.0),
                    egui::Align2::LEFT_TOP,
                    format!("{idx}"),
                    egui::FontId::proportional(9.0),
                    Color32::from_white_alpha(180),
                );
            }

            if app.show_height_labels && tile.height != 0 {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("{}", tile.height),
                    egui::FontId::proportional((cell * 0.35).clamp(8.0, 14.0)),
                    Color32::WHITE,
                );
            }

            if c == app.map.spawn {
                painter.text(
                    rect.right_top() + EVec2::new(-2.0, 1.0),
                    egui::Align2::RIGHT_TOP,
                    "S",
                    egui::FontId::proportional(12.0),
                    Color32::YELLOW,
                );
            }
            if c == app.map.leak {
                painter.text(
                    rect.right_bottom() + EVec2::new(-2.0, -1.0),
                    egui::Align2::RIGHT_BOTTOM,
                    "L",
                    egui::FontId::proportional(12.0),
                    Color32::from_rgb(255, 120, 80),
                );
            }
        }
    }

    // Draw path polyline
    if app.map.path.len() >= 2 {
        let pts: Vec<Pos2> = app
            .map
            .path
            .iter()
            .map(|c| {
                origin
                    + EVec2::new(
                        c.x as f32 * cell + cell * 0.5,
                        c.y as f32 * cell + cell * 0.5,
                    )
            })
            .collect();
        for w in pts.windows(2) {
            painter.line_segment([w[0], w[1]], Stroke::new(2.0, Color32::from_rgb(255, 220, 80)));
        }
    }
}

fn draw_extruded_preview(ui: &mut egui::Ui, map: &MapDef) {
    let avail = ui.available_size();
    let (response, painter) = ui.allocate_painter(avail, Sense::hover());
    let rect = response.rect;

    // Fake isometric projection parameters
    let w = map.width as f32;
    let d = map.depth as f32;
    let scale_x = (rect.width() * 0.85) / (w + d).max(1.0);
    let scale_y = scale_x * 0.5;
    let height_px = scale_x * 0.55;
    let origin = Pos2::new(
        rect.center().x - (w - d) * scale_x * 0.5,
        rect.top() + rect.height() * 0.72,
    );

    let project = |x: f32, y: f32, h: f32| -> Pos2 {
        Pos2::new(
            origin.x + (x - y) * scale_x,
            origin.y + (x + y) * scale_y - h * height_px,
        )
    };

    // Draw back-to-front (by x+y)
    let mut cells: Vec<Cell> = Vec::new();
    for y in 0..map.depth as i32 {
        for x in 0..map.width as i32 {
            cells.push(Cell::new(x, y));
        }
    }
    cells.sort_by_key(|c| c.x + c.y);

    for c in cells {
        let tile = map.get_tile(c).unwrap();
        let h = tile.height.max(0) as f32; // negative heights sit slightly below
        let base_h = if tile.height < 0 {
            tile.height as f32
        } else {
            0.0
        };
        let top_h = tile.height as f32;
        let color = tint_by_height(tile_base_color(tile.kind), tile.height);
        let side = Color32::from_rgb(
            color.r().saturating_sub(30),
            color.g().saturating_sub(30),
            color.b().saturating_sub(30),
        );
        let top = Color32::from_rgb(
            color.r().saturating_add(20).min(255),
            color.g().saturating_add(20).min(255),
            color.b().saturating_add(20).min(255),
        );

        let x0 = c.x as f32;
        let y0 = c.y as f32;
        let x1 = x0 + 1.0;
        let y1 = y0 + 1.0;

        // Bottom and top quads (diamond)
        let b00 = project(x0, y0, base_h);
        let b10 = project(x1, y0, base_h);
        let b11 = project(x1, y1, base_h);
        let b01 = project(x0, y1, base_h);
        let t00 = project(x0, y0, top_h);
        let t10 = project(x1, y0, top_h);
        let t11 = project(x1, y1, top_h);
        let t01 = project(x0, y1, top_h);

        // Only draw vertical faces if height differs from 0 baseline visually
        let _ = h;
        // Right face (x+): b10,b11,t11,t10
        painter.add(egui::Shape::convex_polygon(
            vec![b10, b11, t11, t10],
            side,
            Stroke::NONE,
        ));
        // Left face (y+): b01,b11,t11,t01
        painter.add(egui::Shape::convex_polygon(
            vec![b01, b11, t11, t01],
            Color32::from_rgb(
                side.r().saturating_sub(15),
                side.g().saturating_sub(15),
                side.b().saturating_sub(15),
            ),
            Stroke::NONE,
        ));
        // Top face
        painter.add(egui::Shape::convex_polygon(
            vec![t00, t10, t11, t01],
            top,
            Stroke::new(0.5, Color32::from_black_alpha(40)),
        ));
        // Mark spawn/leak
        if c == map.spawn || c == map.leak {
            let label = if c == map.spawn { "S" } else { "L" };
            let col = if c == map.spawn {
                Color32::YELLOW
            } else {
                Color32::from_rgb(255, 120, 80)
            };
            painter.text(
                t00.lerp(t11, 0.5),
                egui::Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(11.0),
                col,
            );
        }

        // Silence unused bottom corners (kept for clarity of diamond layout).
        let _ = (b00, b01);
    }
}

/// Orthogonal BFS from spawn across Path tiles.
/// Prefers a path that reaches `leak`. If leak is unreachable, returns the
/// path to the farthest discovered Path cell (by BFS distance).
fn rebuild_path_from_spawn(map: &MapDef) -> Result<Vec<Cell>, String> {
    let spawn = map.spawn;
    if !map.in_bounds(spawn) {
        return Err("Spawn out of bounds.".into());
    }
    if map.get_tile(spawn).map(|t| t.kind) != Some(TileKind::Path) {
        return Err("Spawn must be on a Path cell before Rebuild.".into());
    }

    let mut parent: std::collections::HashMap<Cell, Option<Cell>> = std::collections::HashMap::new();
    let mut dist: std::collections::HashMap<Cell, u32> = std::collections::HashMap::new();
    let mut q = VecDeque::new();
    q.push_back(spawn);
    parent.insert(spawn, None);
    dist.insert(spawn, 0);

    let neighbors = |c: Cell| {
        [
            Cell::new(c.x + 1, c.y),
            Cell::new(c.x - 1, c.y),
            Cell::new(c.x, c.y + 1),
            Cell::new(c.x, c.y - 1),
        ]
    };

    while let Some(cur) = q.pop_front() {
        for n in neighbors(cur) {
            if parent.contains_key(&n) {
                continue;
            }
            if !map.in_bounds(n) {
                continue;
            }
            if map.get_tile(n).map(|t| t.kind) != Some(TileKind::Path) {
                continue;
            }
            parent.insert(n, Some(cur));
            dist.insert(n, dist[&cur] + 1);
            q.push_back(n);
        }
    }

    let goal = if parent.contains_key(&map.leak) {
        map.leak
    } else {
        // farthest by BFS distance
        dist.iter()
            .max_by_key(|(_, d)| *d)
            .map(|(c, _)| *c)
            .unwrap_or(spawn)
    };

    let mut path = Vec::new();
    let mut cur = Some(goal);
    let mut guard = HashSet::new();
    while let Some(c) = cur {
        if !guard.insert(c) {
            return Err("Cycle while reconstructing path.".into());
        }
        path.push(c);
        cur = parent.get(&c).copied().flatten();
    }
    path.reverse();
    if path.is_empty() {
        path.push(spawn);
    }
    Ok(path)
}

fn parse_waypoints(text: &str) -> Result<Vec<Cell>, String> {
    let cleaned = text.replace('→', " ").replace("->", " ");
    let mut path = Vec::new();
    for part in cleaned.split(|c: char| c == ';' || c.is_whitespace()) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let nums: Vec<&str> = part.split(',').map(|s| s.trim()).collect();
        if nums.len() != 2 {
            return Err(format!("Bad waypoint '{part}' (want x,y)"));
        }
        let x: i32 = nums[0]
            .parse()
            .map_err(|_| format!("Bad x in '{part}'"))?;
        let y: i32 = nums[1]
            .parse()
            .map_err(|_| format!("Bad y in '{part}'"))?;
        path.push(Cell::new(x, y));
    }
    Ok(path)
}

fn format_path_error(e: &PathValidationError) -> String {
    match e {
        PathValidationError::EmptyPath => "path is empty".into(),
        PathValidationError::OutOfBounds(c) => format!("cell ({}, {}) out of bounds", c.x, c.y),
        PathValidationError::NotOrthogonal { from, to } => format!(
            "({},{}) → ({},{}) is not an orthogonal step",
            from.x, from.y, to.x, to.y
        ),
        PathValidationError::SteepSlope { from, to, delta } => format!(
            "steep slope |Δh|={delta} between ({},{}) and ({},{})",
            from.x, from.y, to.x, to.y
        ),
        PathValidationError::WrongKind { cell, kind } => {
            format!("({}, {}) is {:?} not Path", cell.x, cell.y, kind)
        }
        PathValidationError::SpawnMismatch => "spawn is not the first path waypoint".into(),
        PathValidationError::LeakMismatch => "leak is not the last path waypoint".into(),
    }
}

fn save_map_json(map: &MapDef, path: &PathBuf) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    let json = serde_json::to_string_pretty(map).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

fn load_map_json(path: &PathBuf) -> Result<MapDef, String> {
    let data = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}


#[cfg(test)]
mod workshop_tests {
    use super::*;

    #[test]
    fn rebuild_path_follows_orthogonal_path_to_leak() {
        let mut map = MapDef::stub();
        let path = rebuild_path_from_spawn(&map).expect("rebuild");
        assert_eq!(path.first(), Some(&map.spawn));
        assert_eq!(path.last(), Some(&map.leak));
        assert!(map.validate_path().is_ok() || {
            map.path = path.clone();
            map.validate_path().is_ok()
        });
        map.path = path;
        assert!(map.validate_path().is_ok());
    }

    #[test]
    fn parse_waypoints_arrows() {
        let p = parse_waypoints("0,4 → 1,4 -> 2,4").unwrap();
        assert_eq!(p, vec![Cell::new(0, 4), Cell::new(1, 4), Cell::new(2, 4)]);
    }
}
