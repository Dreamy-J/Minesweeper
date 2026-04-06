use bevy::prelude::*;

use minesweeper::core::events::CellActionEvent;
use minesweeper::core::resources::{Board, DifficultyPreset, GameSession};
use minesweeper::state::game_state::GameStatus;
use minesweeper::systems::game_logic::process_cell_actions;

fn build_logic_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(bevy::state::app::StatesPlugin);
    app.init_state::<GameStatus>();

    let difficulty = DifficultyPreset::Beginner;
    app.insert_resource(Board::new(difficulty));
    app.insert_resource(GameSession::new(difficulty));
    app.add_message::<CellActionEvent>();
    app.add_systems(Update, process_cell_actions);
    app
}

fn flush_updates(app: &mut App) {
    app.update();
    app.update();
}

#[test]
fn first_click_places_mines_and_keeps_safe_zone() {
    let mut app = build_logic_app();

    app.world_mut().write_message(CellActionEvent::reveal(4, 4));
    flush_updates(&mut app);

    let board = app.world().resource::<Board>();
    let session = app.world().resource::<GameSession>();

    assert!(board.mines_placed);
    assert!(!session.first_click);

    let mut safe_zone = board.neighbors(4, 4);
    safe_zone.push((4, 4));
    for (r, c) in safe_zone {
        assert!(!board.cell(r, c).unwrap().is_mine);
    }

    assert_eq!(
        *app.world().resource::<State<GameStatus>>().get(),
        GameStatus::Playing
    );
}

#[test]
fn flag_toggle_updates_counter() {
    let mut app = build_logic_app();

    app.world_mut()
        .write_message(CellActionEvent::toggle_flag(0, 0));
    flush_updates(&mut app);
    assert_eq!(app.world().resource::<GameSession>().flags_placed, 1);

    app.world_mut()
        .write_message(CellActionEvent::toggle_flag(0, 0));
    flush_updates(&mut app);
    assert_eq!(app.world().resource::<GameSession>().flags_placed, 0);
}

#[test]
fn change_difficulty_resets_board_shape() {
    let mut app = build_logic_app();

    app.world_mut()
        .write_message(CellActionEvent::change_difficulty(DifficultyPreset::Expert));
    flush_updates(&mut app);

    let board = app.world().resource::<Board>();
    let session = app.world().resource::<GameSession>();

    assert_eq!(board.rows, 16);
    assert_eq!(board.cols, 30);
    assert_eq!(board.total_mines, 99);
    assert_eq!(session.difficulty, DifficultyPreset::Expert);
    assert_eq!(session.elapsed_seconds, 0.0);
    assert_eq!(
        *app.world().resource::<State<GameStatus>>().get(),
        GameStatus::Playing
    );
}

#[test]
fn revealing_a_mine_sets_defeat_state() {
    let mut app = build_logic_app();

    {
        let mut board = app.world_mut().resource_mut::<Board>();
        board.clear_cells();
        board.mines_placed = true;
        board.cell_mut(0, 0).unwrap().is_mine = true;
    }

    {
        let mut session = app.world_mut().resource_mut::<GameSession>();
        session.first_click = false;
    }

    app.world_mut().write_message(CellActionEvent::reveal(0, 0));
    flush_updates(&mut app);

    assert!(app.world().resource::<GameSession>().frozen);
    assert_eq!(
        *app.world().resource::<State<GameStatus>>().get(),
        GameStatus::Defeat
    );
}

#[test]
fn restart_resets_runtime_counters() {
    let mut app = build_logic_app();

    {
        let mut session = app.world_mut().resource_mut::<GameSession>();
        session.elapsed_seconds = 33.0;
        session.flags_placed = 4;
        session.revealed_safe_cells = 11;
        session.frozen = true;
    }

    app.world_mut().write_message(CellActionEvent::restart());
    flush_updates(&mut app);

    let session = app.world().resource::<GameSession>();
    assert_eq!(session.elapsed_seconds, 0.0);
    assert_eq!(session.flags_placed, 0);
    assert_eq!(session.revealed_safe_cells, 0);
    assert!(!session.frozen);
}
