mod r#parser;
mod r#move;

use parser::parse_chess_notation;
use ux::*;

fn print_chess_board(chess_board:&Vec<Vec<u16>>){
    for row in 0..8{
        println!("------------------------------");
        for column in 0..8{
            print!(" {} ", chess_board[row][column]);
        }
        println!("");
    }
}

fn main() {
    let mut chess_board:Vec<Vec<u16>> = vec![vec![0; 8];8];
    /*
    board looks like this:
      \col  | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
    row\     ___ ___ ___ ___ ___ ___ ___ ___
    0       |___|___|___|___|___|___|___|___|
    1       |___|___|___|___|___|___|___|___|
    2       |___|___|___|___|___|___|___|___|
    3       |___|___|___|___|___|___|___|___|
    4       |___|___|___|___|___|___|___|___|
    5       |___|___|___|___|___|___|___|___|
    6       |___|___|___|___|___|___|___|___|
    7       |___|___|___|___|___|___|___|___|
     */
    let mut game_state:u32 = 0;
    //bit 0:turn 0 is white 1 is black
    //bit 1:white castle short
    //bit 2:white castle long
    //bit 3:black castle short
    //bit 4:black castle long
    //bit 5-10:en passant
    //bit 11-16:half turns for 50 turn rule
    //17-31:turn count

    let w_pawn = 1;
    let w_knight = 2;
    let w_bishop = 3;
    let w_rook = 4;
    let w_queen = 5;
    let w_king = 6;
    let b_pawn = 9;
    let b_knight = 10;
    let b_bishop = 11;
    let b_rook = 12;
    let b_queen = 13;
    let b_king = 14;
    let empty = 0;


    print_chess_board(&chess_board);
    parse_chess_notation(&mut chess_board, &mut game_state, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    print_chess_board(&chess_board);
    println!("{:032b}", game_state);
}
