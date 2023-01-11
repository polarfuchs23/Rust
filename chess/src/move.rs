use ux::*;


pub(crate) fn check_legal_moves(chess_board: &mut Vec<u32>, game_state: &mut u32, coords: &Vec<u8>) -> Vec<Vec<u8>>{
    let piece = (chess_board[coords[0]] & (0b1111 << (7 - coords[1]) * 4)) >> ((7 - coords[1]) * 4);
    let mut legal_moves: Vec<Vec<u8>>;
    match piece {
        0b0000 => {
            if (chess_board[coords[0] - 1] & (0b1111 << (7 - coords[1]) * 4)) >> ((7 - coords[1]) * 4) != 0b1111 {
                legal_moves.push([coords[0] - 1, coords[1]]);
            }
            if coords < 7 & 1{

            }
        },
        0b0001 => {

        }
        _ => {

        }
    }
}