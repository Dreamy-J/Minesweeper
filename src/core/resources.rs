//! 资源定义模块
//!
//! 定义了扫雷游戏中使用的所有 ECS 资源。资源是全局共享的数据，
//! 可以被任何系统访问和修改。

use bevy::prelude::*;

/// 单元格可见性状态
///
/// 表示单元格当前的显示状态，用于控制游戏逻辑和渲染。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellVisibility {
    /// 隐藏状态 - 未揭开且未标记
    Hidden,
    /// 已揭开状态 - 显示数字或地雷
    Revealed,
    /// 已标记状态 - 玩家放置了旗帜
    Flagged,
}

/// 单元格数据
///
/// 存储单个单元格的游戏状态信息，包括是否为地雷、
/// 周围地雷数量以及当前的可见性状态。
#[derive(Debug, Clone)]
pub struct CellData {
    /// 是否为地雷
    pub is_mine: bool,
    /// 周围 8 个邻居中的地雷数量（0-8）
    pub adjacent_mines: u8,
    /// 当前的可见性状态
    pub visibility: CellVisibility,
}

impl Default for CellData {
    /// 创建默认的单元格数据
    ///
    /// 默认情况下，单元格不是地雷，周围地雷数为 0，
    /// 状态为隐藏。
    fn default() -> Self {
        Self {
            is_mine: false,
            adjacent_mines: 0,
            visibility: CellVisibility::Hidden,
        }
    }
}

/// 难度预设枚举
///
/// 定义了三种标准扫雷难度级别，每种难度对应不同的
/// 棋盘尺寸和地雷数量。
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DifficultyPreset {
    /// 初级：9x9 网格，10 个地雷
    #[default]
    Beginner,
    /// 中级：16x16 网格，40 个地雷
    Intermediate,
    /// 专家：16x30 网格，99 个地雷
    Expert,
}

impl DifficultyPreset {
    /// 获取该难度下的行数
    pub fn rows(self) -> u32 {
        match self {
            Self::Beginner => 9,
            Self::Intermediate => 16,
            Self::Expert => 16,
        }
    }

    /// 获取该难度下的列数
    pub fn cols(self) -> u32 {
        match self {
            Self::Beginner => 9,
            Self::Intermediate => 16,
            Self::Expert => 30,
        }
    }

    /// 获取该难度下的地雷总数
    pub fn total_mines(self) -> u32 {
        match self {
            Self::Beginner => 10,
            Self::Intermediate => 40,
            Self::Expert => 99,
        }
    }

    /// 获取难度的显示标签
    pub fn label(self) -> &'static str {
        match self {
            Self::Beginner => "Beginner",
            Self::Intermediate => "Intermediate",
            Self::Expert => "Expert",
        }
    }
}

/// 游戏棋盘资源
///
/// 存储整个扫雷棋盘的状态，包括所有单元格的数据。
/// 提供访问和修改单元格的方法，以及边界检查等辅助功能。
#[derive(Resource, Debug, Clone)]
pub struct Board {
    /// 棋盘行数
    pub rows: u32,
    /// 棋盘列数
    pub cols: u32,
    /// 地雷总数
    pub total_mines: u32,
    /// 是否已经布置了地雷
    pub mines_placed: bool,
    /// 所有单元格的数据，按行优先顺序存储
    cells: Vec<CellData>,
}

impl Board {
    /// 创建一个新的棋盘
    ///
    /// # 参数
    ///
    /// * `difficulty` - 难度级别
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

    /// 重置棋盘为指定难度
    ///
    /// 清空所有单元格数据并重置游戏状态。
    ///
    /// # 参数
    ///
    /// * `difficulty` - 难度级别
    pub fn reset(&mut self, difficulty: DifficultyPreset) {
        self.rows = difficulty.rows();
        self.cols = difficulty.cols();
        self.total_mines = difficulty.total_mines();
        self.mines_placed = false;
        self.cells = vec![CellData::default(); (self.rows * self.cols) as usize];
    }

    /// 清空所有单元格数据（保留棋盘尺寸）
    ///
    /// 用于重新开始游戏时清除旧的地雷布局。
    pub fn clear_cells(&mut self) {
        self.mines_placed = false;
        for cell in &mut self.cells {
            *cell = CellData::default();
        }
    }

    /// 获取单元格总数
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// 获取安全单元格总数（非地雷单元格数）
    ///
    /// 用于判断胜利条件。
    pub fn safe_cell_count(&self) -> u32 {
        self.rows * self.cols - self.total_mines
    }

    /// 检查坐标是否在棋盘范围内
    ///
    /// # 参数
    ///
    /// * `row` - 行号（i32 允许负数检查）
    /// * `col` - 列号（i32 允许负数检查）
    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && row < self.rows as i32 && col < self.cols as i32
    }

    /// 将行列坐标转换为一维数组索引
    ///
    /// # 参数
    ///
    /// * `row` - 行号
    /// * `col` - 列号
    pub fn index(&self, row: u32, col: u32) -> usize {
        (row * self.cols + col) as usize
    }

    /// 获取指定位置的单元格数据（只读）
    ///
    /// 如果坐标越界，返回 `None`。
    pub fn cell(&self, row: u32, col: u32) -> Option<&CellData> {
        if !self.in_bounds(row as i32, col as i32) {
            return None;
        }
        self.cells.get(self.index(row, col))
    }

    /// 获取指定位置的单元格数据（可修改）
    ///
    /// 如果坐标越界，返回 `None`。
    pub fn cell_mut(&mut self, row: u32, col: u32) -> Option<&mut CellData> {
        if !self.in_bounds(row as i32, col as i32) {
            return None;
        }
        let idx = self.index(row, col);
        self.cells.get_mut(idx)
    }

    /// 获取指定位置的所有邻居坐标
    ///
    /// 返回最多 8 个邻居坐标（不包括自身），
    /// 自动过滤越界的坐标。
    ///
    /// # 参数
    ///
    /// * `row` - 中心行号
    /// * `col` - 中心列号
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

    /// 迭代所有单元格数据（只读）
    pub fn iter_cells(&self) -> impl Iterator<Item = &CellData> {
        self.cells.iter()
    }

    /// 揭开所有地雷
    ///
    /// 用于游戏失败时显示所有地雷位置。
    pub fn reveal_all_mines(&mut self) {
        for cell in &mut self.cells {
            if cell.is_mine {
                cell.visibility = CellVisibility::Revealed;
            }
        }
    }
}

/// 游戏配置资源
///
/// 存储游戏的显示配置参数。
#[derive(Resource, Debug, Clone, Copy)]
pub struct GameConfig {
    /// 每个单元格的像素尺寸（正方形）
    pub cell_size: f32,
}

impl Default for GameConfig {
    /// 默认配置：单元格大小为 32 像素
    fn default() -> Self {
        Self { cell_size: 32.0 }
    }
}

/// 游戏会话资源
///
/// 存储当前游戏会话的运行状态信息。
#[derive(Resource, Debug, Clone, Copy)]
pub struct GameSession {
    /// 当前难度级别
    pub difficulty: DifficultyPreset,
    /// 已经过的游戏时间（秒）
    pub elapsed_seconds: f32,
    /// 当前放置的旗帜数量
    pub flags_placed: u32,
    /// 已揭开的安全单元格数量
    pub revealed_safe_cells: u32,
    /// 是否为第一次点击（用于安全区生成）
    pub first_click: bool,
    /// 游戏是否冻结（胜利或失败后停止响应输入）
    pub frozen: bool,
}

impl GameSession {
    /// 创建新的游戏会话
    ///
    /// # 参数
    ///
    /// * `difficulty` - 难度级别
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

    /// 重置运行时状态
    ///
    /// 用于重新开始游戏时清除旧的游戏状态，
    /// 但不改变难度设置。
    pub fn reset_runtime(&mut self) {
        self.elapsed_seconds = 0.0;
        self.flags_placed = 0;
        self.revealed_safe_cells = 0;
        self.first_click = true;
        self.frozen = false;
    }
}

/// 单元格实体映射资源
///
/// 存储单元格实体 ID 的列表，用于快速查找和更新
/// 单元格的视觉表现。实体按行优先顺序存储。
#[derive(Resource, Default)]
pub struct CellEntityMap {
    /// 单元格实体 ID 列表
    pub entities: Vec<Entity>,
}

impl CellEntityMap {
    /// 清空所有实体映射
    pub fn clear(&mut self) {
        self.entities.clear();
    }
}
