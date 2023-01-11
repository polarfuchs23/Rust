use std::collections::HashMap;
use ux::*;


pub(crate) fn parse_chess_notation(chess_board: &mut Vec<u32>, game_state: &mut u32, chess_notation: &str){
    *game_state = 0;
    let piece_map = HashMap::from([
        ('P', u4::new(0b0000)),
        ('N', u4::new(0b0001)),
        ('B', u4::new(0b0010)),
        ('R', u4::new(0b0011)),
        ('Q', u4::new(0b0100)),
        ('K', u4::new(0b0101)),
        ('p', u4::new(0b1000)),
        ('n', u4::new(0b1001)),
        ('b', u4::new(0b1010)),
        ('r', u4::new(0b1011)),
        ('q', u4::new(0b1100)),
        ('k', u4::new(0b1101))
    ]);
    let castle_map = HashMap::from([
        ('K', 0b10),
        ('Q', 0b100),
        ('k', 0b1000),
        ('q', 0b10000)
    ]);

    let mut row:usize = 0;
    let mut column:usize = 0;
    let mut meaning:i16 = 0;
    '_outer: for c in chess_notation.chars(){
        println!("{}", c);
        if c == '/'{
            row += 1;
            column = 0;
        }
        else if c == ' ' {
            meaning += 1;
        }
        else if meaning == 0 {
            println!("{}", c);
            if "12345678".contains(c){
                println!("{}", c.to_digit(10).unwrap());
                for i in 0..(c.to_digit(10).unwrap()){
                    (row, column) = (row + (column / 8), column % 8);
                    if row == 8{
                        println!("break");
                        break '_outer
                    }
                    println!("row {}", row);
                    chess_board[row] = chess_board[row] | (0b1111 << (7 - column) * 4);
                    column += 1;
                    println!("{}", i);
                }
            }
            else {
                (row, column) = (row + (column / 8), column % 8);
                if row == 8{
                    println!("break");
                    break '_outer
                }
                chess_board[row] = (chess_board[row] & !(0b1111 << (7 - column) * 4)) | (u32::from(piece_map[&c]) << (7 - column) * 4);
                column += 1;
            }
        }
        else if meaning == 1{
            if c == 'w'{
                *game_state += 0b1;
            }
        }
        else if meaning == 2{
            *game_state += castle_map[&c];
        }
        else if meaning == 3{
            if c != '-' {
                *game_state += (c as u32 - 97) << 8;
            }
            else { meaning += 1; }
        }
        else if meaning == 4{
            *game_state += (c as u32 - 97) << 5;
        }
        else if meaning == 5{
            *game_state += c.to_digit(10).unwrap() << 11;
        }
        else if meaning == 6{
            *game_state += c.to_digit(10).unwrap() << 17;
        }
    }
}