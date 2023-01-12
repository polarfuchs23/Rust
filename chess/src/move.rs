use ux::*;


pub(crate) fn check_legal_moves(chess_board: &mut Vec<Vec<u16>>, game_state: &mut u32, coords: &Vec<u8>) -> Vec<Vec<u8>>{
    let coord_row = &coords[0];
    let coord_col = &coords[1];
    let piece = chess_board[coord_row][coord_col];
    let mut legal_moves: Vec<Vec<u8>> = vec![];
    match piece {
        1 => {
            if chess_board[*coord_row - 1] == 0 {
                legal_moves.push(vec![*coord_row - 1, *coord_col]);
            }
            if (*coord_col < 7) & (chess_board[*coord_row - 1][*coord_col + 1] > 8) {
                legal_moves.push(vec![*coord_row - 1, *coord_col + 1]);
            }
            if (*coord_col > 0) & (chess_board[*coord_row - 1][*coord_col - 1] > 8) {
                legal_moves.push(vec![*coord_row - 1, *coord_col + 1]);
            }
        },
        2 | 10 => {

        },
        _ => {

        }
    }
    legal_moves
}