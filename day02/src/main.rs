use utils;

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

fn first_half(lines: &Vec<&str>) -> u32 {

    let mut points: u32 = 0;

    for line in lines {
        let bytes = line.as_bytes();
        let opponent_move: u8 = bytes[0] - A + 1;
        let my_move: u8 = bytes[2] - X + 1;

        let round_pts = game_points(opponent_move, my_move);
        points += round_pts;
        points += my_move as u32;

//        println!("{} vs {} => {} + {}", opponent_move, my_move, round_pts, my_move);
    }

    return points;
}

const LOSE: u8 = 'X' as u8;
const WIN: u8 = 'Z' as u8;


fn choose_my_move(opponent_move: u8, expected_result: u8) -> u8 {

    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    return match expected_result {
        LOSE => (opponent_move + 1) % 3 + 1,
        WIN => (opponent_move) % 3 + 1,
        _ => opponent_move
    };
}

fn second_half(lines: &Vec<&str>) -> u32 {

    let mut points: u32 = 0;

    for line in lines {
        let bytes = line.as_bytes();
        let opponent_move: u8 = bytes[0] - A + 1;
        let my_move: u8 = choose_my_move(opponent_move, bytes[2]);

        let round_pts = game_points(opponent_move, my_move);
        points += round_pts;
        points += my_move as u32;

//        println!("{} vs {} => {} + {}", opponent_move, my_move, round_pts, my_move);
    }

    return points;
}

fn main() -> std::io::Result<()> {

    let content = utils::load_data()?;
    let lines: Vec<&str> = content.lines().collect();

    // let it = "A Y\nB X\nC Z".lines();
    let pts1: u32 = first_half(&lines);
    println!("1) Total points {}", pts1);

    //second_half("A Y\nB X\nC Z".lines());
    let pts2: u32 = second_half(&lines);
    println!("2) Total points {}", pts2);


    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_first_half() {
        let lines = "A Y\nB X\nC Z".lines().collect();
        let points: u32 = first_half(&lines);
        assert_eq!(points, 15)
    }

    #[test]
    fn test_second_half() {
        let lines = "A Y\nB X\nC Z".lines().collect();
        let points: u32 = second_half(&lines);
        assert_eq!(points, 12)
    }
}