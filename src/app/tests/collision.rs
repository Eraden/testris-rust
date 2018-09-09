use app::tetris_game::*;
use app::tests::utils::TestableApp;

#[test]
pub fn check_collision_no_collision() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(current_block);
    let next_pos = Pos { x: 0, y: 0 };
    assert_eq!(app.check_collision(next_pos), CollisionResult::NoCollision);
}

#[test]
pub fn check_collision_move_to_end_of_row() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(current_block);
    let next_pos = Pos { x: app.width, y: 0 };
    assert_eq!(app.check_collision(next_pos), CollisionResult::Collide);
}

#[test]
pub fn check_collision_move_to_end_of_column() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(current_block);
    let next_pos = Pos { x: 0, y: app.height };
    assert_eq!(app.check_collision(next_pos), CollisionResult::Collide);
}

#[test]
pub fn check_collision_block_moved_by_x1() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        0, 1, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(current_block);
    let next_pos = Pos { x: app.width - 1, y: 0 };
    assert_eq!(app.check_collision(next_pos), CollisionResult::Collide);
}

#[test]
pub fn check_collision_block_moved_by_y1() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        0, 0, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(current_block);
    let next_pos = Pos { x: 0, y: app.height - 1 };
    assert_eq!(app.check_collision(next_pos), CollisionResult::Collide);
}

