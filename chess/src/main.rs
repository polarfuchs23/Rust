mod parser;
use parser::parse_chess_notation;
use ux::*;

fn print_chess_board(chess_board:&Vec<u32>){
    for row in 0..8{
        println!("------------------------------");
        for column in 0..8{
            print!(" {:04b} ", (chess_board[row] & (0b1111 << (7 - column) * 4)) >> (7 - column) * 4);
        }
        println!("");
    }
}

fn main() {
    let mut chess_board:Vec<u32> = vec![0; 8];
    let mut game_state:u32 = 0;
    //bit 0:turn 0 is white 1 is black
    //bit 1:white castle short
    //bit 2:white castle long
    //bit 3:black castle short
    //bit 4:black castle long
    //bit 5-10:en passant
    //bit 11-16:half turns for 50 turn rule
    //17-31:turn count

    let w_pawn = u4::new(0b0000);
    let w_knight = u4::new(0b0001);
    let w_bishop = u4::new(0b0010);
    let w_rook = u4::new(0b0011);
    let w_queen = u4::new(0b0100);
    let w_king = u4::new(0b0101);
    let b_pawn = u4::new(0b1000);
    let b_knight = u4::new(0b1001);
    let b_bishop = u4::new(0b1010);
    let b_rook = u4::new(0b1011);
    let b_queen = u4::new(0b1100);
    let b_king = u4::new(0b1101);
    let empty = u4::new(0b1111);


    print_chess_board(&chess_board);
    parse_chess_notation(&mut chess_board, &mut game_state, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    print_chess_board(&chess_board);
    println!("{:032b}", game_state);
    println!("Hello, world!");
}
