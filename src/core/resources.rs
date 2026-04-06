use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellVisibility {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone)]
pub struct CellData {
    pub is_mine: bool,
    pub adjacent_mines: u8,
    pub visibility: CellVisibility,
}

impl Default for CellData {
    fn default() -> Self {
        Self {
            is_mine: false,
            adjacent_mines: 0,
            visibility: CellVisibility::Hidden,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DifficultyPreset {
    #[default]
    Beginner,
    Intermediate,
    Expert,
}

impl DifficultyPreset {
    pub fn rows(self) -> u32 {
        match self {
            Self::Beginner => 9,
            Self::Intermediate => 16,
            Self::Expert => 16,
        }
    }

    pub fn cols(self) -> u32 {
        match self {
            Self::Beginner => 9,
            Self::Intermediate => 16,
            Self::Expert => 30,
        }
    }

    pub fn total_mines(self) -> u32 {
        match self {
            Self::Beginner => 10,
            Self::Intermediate => 40,
            Self::Expert => 99,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Expert => "Expert",
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Board {
    pub rows: u32,
    pub cols: u32,
    pub total_mines: u32,
    pub mines_placed: bool,
    cells: Vec<CellData>,
}

impl Board {
    pub fn new(difficulty: DifficultyPreset) -> Self {
        let mut board = Self {
            rows: 0,
            cols: 0,
            total_mines: 0,
            mines_placed: false,
            cells: Vec::new(),
        };
        board.reset(difficulty);
        board
    }

    pub fn reset(&mut self, difficulty: DifficultyPreset) {
        self.rows = difficulty.rows();
        self.cols = difficulty.cols();
        self.total_mines = difficulty.total_mines();
        self.mines_placed = false;
        self.cells = vec![CellData::default(); (self.rows * self.cols) as usize];
    }

    pub fn clear_cells(&mut self) {
        self.mines_placed = false;
        for cell in &mut self.cells {
            *cell = CellData::default();
        }
    }

    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    pub fn safe_cell_count(&self) -> u32 {
        self.rows * self.cols - self.total_mines
    }

    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && row < self.rows as i32 && col < self.cols as i32
    }

    pub fn index(&self, row: u32, col: u32) -> usize {
        (row * self.cols + col) as usize
    }

    pub fn cell(&self, row: u32, col: u32) -> Option<&CellData> {
        if !self.in_bounds(row as i32, col as i32) {
            return None;
        }
        self.cells.get(self.index(row, col))
    }

    pub fn cell_mut(&mut self, row: u32, col: u32) -> Option<&mut CellData> {
        if !self.in_bounds(row as i32, col as i32) {
            return None;
        }
        let idx = self.index(row, col);
        self.cells.get_mut(idx)
    }

    pub fn neighbors(&self, row: u32, col: u32) -> Vec<(u32, u32)> {
        let mut out = Vec::with_capacity(8);
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nr = row as i32 + dr;
                let nc = col as i32 + dc;
                if self.in_bounds(nr, nc) {
                    out.push((nr as u32, nc as u32));
                }
            }
        }
        out
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = &CellData> {
        self.cells.iter()
    }

    pub fn reveal_all_mines(&mut self) {
        for cell in &mut self.cells {
            if cell.is_mine {
                cell.visibility = CellVisibility::Revealed;
            }
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct GameConfig {
    pub cell_size: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { cell_size: 32.0 }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct GameSession {
    pub difficulty: DifficultyPreset,
    pub elapsed_seconds: f32,
    pub flags_placed: u32,
    pub revealed_safe_cells: u32,
    pub first_click: bool,
    pub frozen: bool,
}

impl GameSession {
    pub fn new(difficulty: DifficultyPreset) -> Self {
        Self {
            difficulty,
            elapsed_seconds: 0.0,
            flags_placed: 0,
            revealed_safe_cells: 0,
            first_click: true,
            frozen: false,
        }
    }

    pub fn reset_runtime(&mut self) {
        self.elapsed_seconds = 0.0;
        self.flags_placed = 0;
        self.revealed_safe_cells = 0;
        self.first_click = true;
        self.frozen = false;
    }
}

#[derive(Resource, Default)]
pub struct CellEntityMap {
    pub entities: Vec<Entity>,
}

impl CellEntityMap {
    pub fn clear(&mut self) {
        self.entities.clear();
    }
}
