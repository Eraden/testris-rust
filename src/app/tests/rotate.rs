use app::tetris_game::*;
use app::tests::utils::TestableApp;

#[test]
pub fn check_rotate_4() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(&current_block);
    assert_eq!(current_block, app.test_current_block());
    app.rotate_current();
    let current: [u8; 16] = app.test_current_block();
    let expected: [u8; 16] = [
        0, 1, 1, 0,
        1, 1, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    assert_eq!(current, expected);
}

#[test]
pub fn check_rotate_line() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 0, 0, 0,
    ];
    app.test_inject_current_block(&current_block);
    assert_eq!(current_block, app.test_current_block());
    app.rotate_current();
    let current: [u8; 16] = app.test_current_block();
    let expected: [u8; 16] = [
        1, 1, 1, 1,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    assert_eq!(current, expected);
}

#[test]
pub fn check_rotate_l() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 1, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(&current_block);
    assert_eq!(current_block, app.test_current_block());
    app.rotate_current();
    let current: [u8; 16] = app.test_current_block();
    let expected: [u8; 16] = [
        0, 0, 1, 0,
        1, 1, 1, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    assert_eq!(current, expected);
}

#[test]
pub fn check_rotate_twice_4() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(&current_block);
    assert_eq!(current_block, app.test_current_block());
    app.rotate_current();
    app.rotate_current();
    let current: [u8; 16] = app.test_current_block();
    let expected: [u8; 16] = [
        1, 0, 0, 0,
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0,
    ];
    assert_eq!(current, expected);
}

#[test]
pub fn check_rotate_twice_l() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 0, 0, 0,
        1, 0, 0, 0,
        1, 1, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(&current_block);
    assert_eq!(current_block, app.test_current_block());
    app.rotate_current();
    app.rotate_current();
    let current: [u8; 16] = app.test_current_block();
    let expected: [u8; 16] = [
        1, 1, 0, 0,
        0, 1, 0, 0,
        0, 1, 0, 0,
        0, 0, 0, 0,
    ];
    assert_eq!(current, expected);
}
