use std::fs;
use std::env;

const A: u8 = 'A' as u8;
const X: u8 = 'X' as u8;

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

fn game_points(opponent_move: u8, my_move: u8) -> u32 {

    if opponent_move == my_move {
        return 3;
    } else {

        return match (opponent_move, my_move) {
            (ROCK, PAPER) => 6,
            (PAPER, SCISSORS) => 6,
            (SCISSORS, ROCK) => 6,
            (_, _) => 0
        };
    }
}

fn first_half(lines: std::str::Lines) {

    let mut points: u32 = 0;

    for line in lines {
        let bytes = line.as_bytes();
        let opponent_move: u8 = bytes[0] - A + 1;
        let my_move: u8 = bytes[2] - X + 1;

        let round_pts = game_points(opponent_move, my_move);
        points += round_pts;
        points += my_move as u32;

        println!("{} vs {} => {} + {}", opponent_move, my_move, round_pts, my_move);
    }

    println!("1) Total points {}", points);
}


// fn second_half(file: &File) -> std::io::Result<()> {

//     let reader = BufReader::new(file);

//     for line in reader.lines() {

//     }
//     Ok(())
// }

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("Not enough command line parameters"); }

    let filename: &String = &args[1];
    println!("\nIncoming path: {}", filename);

    // let it = "A Y\nB X\nC Z".lines();
    let file_content = fs::read_to_string(filename)?;
    let it = file_content.lines();
    first_half(it);

    // file.rewind()?;
    // second_half(&file)?;

    Ok(())
}