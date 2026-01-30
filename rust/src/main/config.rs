use crate::{Board, Field};

fn make_controller(x_start: usize, y_start: usize, board: Board) {
        ..
}
fn make_field(is_bomb: bool, is_opened: bool, is_flag: bool) -> Field {
    Field { is_bomb: is_bomb, is_opened: is_opened, is_flag: is_flag}
}
fn generate_board(x_size: usize, y_size: 
    usize, x_start: usize, y_start: usize, bomb_count: usize) -> Board {
        Board(x_size, y_start, x_start, y_start, bomb_count)
}
