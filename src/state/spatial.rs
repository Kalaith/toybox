//! Uniform spatial grid over the shop floor for fast toy proximity queries.
//!
//! Tracks every toy with a meaningful world position (not carried, not a
//! consumed repair part). Mutation sites re-file toys via `sync_toy`; the
//! session constructor and save-load path call `rebuild`.

use super::ToyState;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct ToySpatialGrid {
    cell_size: f32,
    columns: usize,
    rows: usize,
    cells: Vec<Vec<usize>>,
    toy_cells: Vec<Option<usize>>,
}

impl ToySpatialGrid {
    pub fn new(room_width: f32, room_height: f32, cell_size: f32) -> Self {
        let cell_size = cell_size.max(0.25);
        let columns = ((room_width / cell_size).ceil() as usize).max(1);
        let rows = ((room_height / cell_size).ceil() as usize).max(1);
        Self {
            cell_size,
            columns,
            rows,
            cells: vec![Vec::new(); columns * rows],
            toy_cells: Vec::new(),
        }
    }

    pub fn rebuild(&mut self, toys: &[ToyState]) {
        for cell in &mut self.cells {
            cell.clear();
        }
        self.toy_cells.clear();
        self.toy_cells.resize(toys.len(), None);
        for (index, toy) in toys.iter().enumerate() {
            self.sync_toy(index, toy);
        }
    }

    /// Re-file one toy after any change to its position, carry, or repair state.
    pub fn sync_toy(&mut self, index: usize, toy: &ToyState) {
        if index >= self.toy_cells.len() {
            self.toy_cells.resize(index + 1, None);
        }
        let next = grid_tracks_toy(toy).then(|| self.cell_index(toy.position.to_vec2()));
        let previous = self.toy_cells[index];
        if previous == next {
            return;
        }
        if let Some(cell) = previous {
            self.cells[cell].retain(|&existing| existing != index);
        }
        if let Some(cell) = next {
            self.cells[cell].push(index);
        }
        self.toy_cells[index] = next;
    }

    /// Indices of tracked toys in cells overlapping the radius around
    /// `position`, ascending. Candidates only — callers still check each toy.
    pub fn indices_near(&self, position: Vec2, radius: f32) -> Vec<usize> {
        let min_column = self.column_at(position.x - radius);
        let max_column = self.column_at(position.x + radius);
        let min_row = self.row_at(position.y - radius);
        let max_row = self.row_at(position.y + radius);

        let mut indices = Vec::new();
        for row in min_row..=max_row {
            for column in min_column..=max_column {
                indices.extend_from_slice(&self.cells[row * self.columns + column]);
            }
        }
        indices.sort_unstable();
        indices
    }

    fn cell_index(&self, position: Vec2) -> usize {
        self.row_at(position.y) * self.columns + self.column_at(position.x)
    }

    fn column_at(&self, x: f32) -> usize {
        ((x / self.cell_size).floor().max(0.0) as usize).min(self.columns - 1)
    }

    fn row_at(&self, y: f32) -> usize {
        ((y / self.cell_size).floor().max(0.0) as usize).min(self.rows - 1)
    }
}

fn grid_tracks_toy(toy: &ToyState) -> bool {
    !toy.is_held && !toy.is_consumed_repair_part()
}
