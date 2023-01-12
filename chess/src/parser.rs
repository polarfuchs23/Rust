use std::collections::HashMap;
use ux::*;


pub(crate) fn parse_chess_notation(chess_board: &mut Vec<Vec<u16>>, game_state: &mut u32, chess_notation: &str){
    *game_state = 0;
    let piece_map = HashMap::from([
        ('P', 1),
        ('N', 2),
        ('B', 3),
        ('R', 4),
        ('Q', 5),
        ('K', 6),
        ('p', 9),
        ('n', 10),
        ('b', 11),
        ('r', 12),
        ('q', 13),
        ('k', 14)
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
                    chess_board[row][column] = 0;
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
                chess_board[row][column] = piece_map[&c];
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