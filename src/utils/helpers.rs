use crate::core::resources::GameConfig;

// 辅助工具函数

/// 将行列坐标转换为世界坐标
pub fn grid_to_world(i: u32, j: u32, config: &GameConfig) -> (f32, f32) {
    let x = config.half_width - j as f32 * config.cell_size;
    let y = config.half_height - i as f32 * config.cell_size;
    (x, y)
}

/// 将世界坐标转换为数组坐标
pub fn world_to_vec(x: f32, y: f32, config: &GameConfig) -> (i32, i32) {
    let i = ((config.half_height - y) / config.cell_size) as i32;
    let j = ((config.half_width + x) / config.cell_size) as i32;
    (i, j)
}
