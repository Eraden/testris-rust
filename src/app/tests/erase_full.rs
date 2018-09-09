use app::tetris_game::*;
use app::tests::utils::TestableApp;

#[test]
pub fn check_need_erase_one_line() {
    let mut app = TetrisGame::new();
    app.test_inject_grid_line(19, &[1; 11]);
    app.remove_full_lines();

    {
        let actual: [u8; 11] = app.test_current_grid_line(19);
        assert_eq!(actual, [0; 11]);
    }
    assert_eq!(app.score, 10);
}

#[test]
pub fn check_need_erase_two_line() {
    let mut app = TetrisGame::new();
    app.test_inject_grid_line(19, &[1; 11]);
    app.test_inject_grid_line(18, &[1; 11]);
    app.remove_full_lines();

    {
        let actual: [u8; 11] = app.test_current_grid_line(18);
        assert_eq!(actual, [0; 11]);
    }
    {
        let actual: [u8; 11] = app.test_current_grid_line(19);
        assert_eq!(actual, [0; 11]);
    }
    assert_eq!(app.score, 20);
}

#[test]
pub fn check_need_erase_irregular() {
    let mut app = TetrisGame::new();
    let one_gap = [1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let two_gaps = [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1];
    app.test_inject_grid_line(19, &[1; 11]);
    app.test_inject_grid_line(18, &[1; 11]);
    app.test_inject_grid_line(17, &one_gap);
    app.test_inject_grid_line(16, &[1; 11]);
    app.test_inject_grid_line(15, &two_gaps);
    app.test_inject_grid_line(14, &[1; 11]);
    app.remove_full_lines();

    {
        let actual: [u8; 11] = app.test_current_grid_line(18);
        assert_eq!(actual, two_gaps);
    }
    {
        let actual: [u8; 11] = app.test_current_grid_line(19);
        assert_eq!(actual, one_gap);
    }
    assert_eq!(app.score, 40);
}
