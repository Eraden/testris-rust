use app::tetris_game::*;
use app::tests::utils::TestableApp;

#[test]
fn check_complex_block() {
    let mut app = TetrisGame::new();
    app.pos.x = 0;
    app.pos.y = 0;
    let current_block: [u8; 16] = [
        1, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(&current_block);
    // y = 0
    assert_eq!(app.is_taken(0, 0), AvailabilityResult::Taken);
    assert_eq!(app.is_taken(1, 0), AvailabilityResult::Taken);
    assert_eq!(app.is_taken(2, 0), AvailabilityResult::Free);
    assert_eq!(app.is_taken(3, 0), AvailabilityResult::Free);
    // y = 1
    assert_eq!(app.is_taken(0, 1), AvailabilityResult::Taken);
    assert_eq!(app.is_taken(1, 1), AvailabilityResult::Free);
    assert_eq!(app.is_taken(2, 1), AvailabilityResult::Free);
    assert_eq!(app.is_taken(3, 1), AvailabilityResult::Free);
}

#[test]
fn check_moved_cursor_complex_block() {
    let mut app = TetrisGame::new();
    let current_block: [u8; 16] = [
        1, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    app.test_inject_current_block(&current_block);
    app.pos.x = 2;
    app.pos.y = 0;
    // y = 0
    assert_eq!(app.is_taken(0, 0), AvailabilityResult::Free);
    assert_eq!(app.is_taken(1, 0), AvailabilityResult::Free);
    assert_eq!(app.is_taken(2, 0), AvailabilityResult::Taken);
    assert_eq!(app.is_taken(3, 0), AvailabilityResult::Taken);
    // y = 1
    assert_eq!(app.is_taken(0, 1), AvailabilityResult::Free);
    assert_eq!(app.is_taken(1, 1), AvailabilityResult::Free);
    assert_eq!(app.is_taken(2, 1), AvailabilityResult::Taken);
    assert_eq!(app.is_taken(3, 1), AvailabilityResult::Free);
}

