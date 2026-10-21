//! Loose desktop icons: transparent, event-driven surfaces with native empty-space input.

use super::*;
use crate::layout::{CellRect, GridMetrics};
use pecofence_core::DesktopPosition;

impl App {
    pub(super) fn release_to_desktop(&mut self, items: &[ItemId], point: Option<(i32, i32)>) {
        let (portal, desktop): (Vec<_>, Vec<_>) = items
            .iter()
            .copied()
            .partition(|id| self.state.is_portal_item(*id));
        if !portal.is_empty() {
            self.move_portal_items_to_desktop(&portal, uuid::Uuid::nil(), point);
        }
        let position = point.and_then(|(x, y)| {
            let work = self
                .state
                .work_areas
                .iter()
                .find(|w| x >= w.left && x < w.right && y >= w.top && y < w.bottom)
                .or_else(|| self.state.work_areas.first())?;
            let m = GridMetrics::for_icon_size(self.state.config.settings.icon_size, 2)
                .with_line_h(self.ctx.chrome.label_line_h());
            let width = (work.right - work.left).max(1) as f32;
            let height = (work.bottom - work.top).max(1) as f32;
            let scale = work.scale();
            Some(DesktopPosition {
                monitor: work.device_path.clone(),
                x: ((x - work.left) as f32 - m.cell_w * scale / 2.0)
                    .clamp(0.0, (width - m.cell_w * scale).max(0.0))
                    / width,
                y: ((y - work.top) as f32 - (m.icon_top + m.icon / 2.0) * scale)
                    .clamp(0.0, (height - m.cell_h * scale).max(0.0))
                    / height,
            })
        });
        if self.state.release_items(&desktop, position) > 0 {
            self.refresh_all();
            self.schedule_save();
            self.push_workspace_summary();
        }
    }

    pub(super) fn refresh_desktop(&mut self) {
        if !self.state.config.settings.hide_real_icons || self.no_hide_icons {
            self.desktop_windows.clear();
            return;
        }
        let works = self.state.work_areas.clone();
        let Some(first) = works.first() else {
            return;
        };
        self.desktop_windows
            .retain(|key, _| works.iter().any(|w| &w.device_path == key));
        let placements = self.state.config.layouts[self.state.layout]
            .desktop_items
            .clone();
        let loose = self.state.loose_items();
        let mut groups: HashMap<String, Vec<ItemView>> = HashMap::new();
        for item in loose {
            let monitor = placements
                .get(&item.id)
                .and_then(Option::as_ref)
                .map(|p| p.monitor.as_str())
                .filter(|id| works.iter().any(|w| w.device_path == *id))
                .unwrap_or(&first.device_path);
            let path = PathBuf::from(item.key.as_path().unwrap_or(""));
            let (_, icon_only) = crate::icons::icon_key_for(&path, item.is_folder, item.mtime);
            groups
                .entry(monitor.to_string())
                .or_default()
                .push(ItemView::new(
                    item.id,
                    path,
                    item.display_name.clone(),
                    item.is_folder,
                    item.icon_key.clone(),
                    icon_only,
                    item.mtime,
                    item.size,
                ));
        }
        let fence_rects: Vec<_> = self
            .state
            .host_fences()
            .iter()
            .map(|f| self.state.fence_px_rect(f))
            .collect();
        let m = GridMetrics::for_icon_size(self.state.config.settings.icon_size, 2)
            .with_line_h(self.ctx.chrome.label_line_h());
        let mut saved = Vec::new();
        for work in &works {
            let items = groups.remove(&work.device_path).unwrap_or_default();
            if items.is_empty() {
                self.desktop_windows.remove(&work.device_path);
                continue;
            }
            let scale = work.scale();
            let width = (work.right - work.left).max(1) as f32;
            let height = (work.bottom - work.top).max(1) as f32;
            let wd = width / scale;
            let hd = height / scale;
            let mut cells = Vec::<CellRect>::new();
            // Reserve saved positions before finding spots for newly arrived icons.
            let reserved: Vec<_> = items
                .iter()
                .filter_map(|item| {
                    let p = placements.get(&item.id)?.as_ref()?;
                    Some(CellRect {
                        x: (p.x * wd).clamp(0.0, (wd - m.cell_w).max(0.0)),
                        y: (p.y * hd).clamp(0.0, (hd - m.cell_h).max(0.0)),
                        w: m.cell_w,
                        h: m.cell_h,
                    })
                })
                .collect();
            let overlaps = |a: CellRect, b: CellRect| {
                a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h
            };
            for item in &items {
                let existing = placements.get(&item.id).and_then(Option::as_ref);
                let mut cell = existing.map(|p| CellRect {
                    x: (p.x * wd).clamp(0.0, (wd - m.cell_w).max(0.0)),
                    y: (p.y * hd).clamp(0.0, (hd - m.cell_h).max(0.0)),
                    w: m.cell_w,
                    h: m.cell_h,
                });
                if cell.is_none() {
                    'place: for col in 0..((wd / (m.cell_w + 8.0)).floor() as usize).max(1) {
                        for row in 0..((hd / (m.cell_h + 8.0)).floor() as usize).max(1) {
                            let candidate = CellRect {
                                x: 8.0 + col as f32 * (m.cell_w + 8.0),
                                y: 8.0 + row as f32 * (m.cell_h + 8.0),
                                w: m.cell_w,
                                h: m.cell_h,
                            };
                            let on_fence = fence_rects.iter().any(|r| {
                                let x = work.left as f32 + candidate.x * scale;
                                let y = work.top as f32 + candidate.y * scale;
                                x < r.right as f32
                                    && (r.left as f32) < x + candidate.w * scale
                                    && y < r.bottom as f32
                                    && (r.top as f32) < y + candidate.h * scale
                            });
                            if !on_fence
                                && !cells
                                    .iter()
                                    .chain(reserved.iter())
                                    .any(|c| overlaps(candidate, *c))
                            {
                                cell = Some(candidate);
                                break 'place;
                            }
                        }
                    }
                }
                let cell = cell.unwrap_or(CellRect {
                    x: 8.0,
                    y: 8.0,
                    w: m.cell_w,
                    h: m.cell_h,
                });
                saved.push((
                    item.id,
                    DesktopPosition {
                        monitor: work.device_path.clone(),
                        x: cell.x / wd,
                        y: cell.y / hd,
                    },
                ));
                cells.push(cell);
            }
            let bounds = RECT {
                left: work.left,
                top: work.top,
                right: work.right,
                bottom: work.bottom,
            };
            if !self.desktop_windows.contains_key(&work.device_path) {
                match FenceWindow::create_desktop(
                    &self.ctx,
                    uuid::Uuid::new_v4(),
                    bounds,
                    self.state.config.settings.icon_size,
                ) {
                    Ok(w) => {
                        self.desktop_windows.insert(work.device_path.clone(), w);
                    }
                    Err(error) => {
                        tracing::error!(%error, "desktop icon surface failed");
                        continue;
                    }
                }
            }
            if let Some(w) = self.desktop_windows.get(&work.device_path) {
                if w.rect() != bounds {
                    w.set_bounds(bounds);
                }
                w.set_icon_size(self.state.config.settings.icon_size);
                w.set_desktop_items(items, cells);
                w.show(false);
            }
        }
        for (id, position) in saved {
            // Membership in `desktop_items` is what pins a loose icon against the rules'
            // Apply (`plan_rules` skips it as manually placed). Only `release_items` — a
            // drop, a deleted fence's leftovers, "release to desktop" — may create that
            // entry. Auto-placing a newcomer must not create one: the entry would exempt
            // it from `plan_rules` forever and Apply now would never move a loose icon.
            if let Some(entry) = self.state.config.layouts[self.state.layout]
                .desktop_items
                .get_mut(&id)
                && entry.as_ref() != Some(&position)
            {
                *entry = Some(position);
                self.state.mark_dirty();
            }
        }
        if let Some(anchor) = self.anchor.borrow_mut().as_mut() {
            anchor.reanchor("desktop icon surfaces");
        }
    }
}
