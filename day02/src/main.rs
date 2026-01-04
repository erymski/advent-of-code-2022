const A: u8 = b'A';
const X: u8 = b'X';

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
            (_, _) => 0,
        };
    }
}

fn first_half(pairs: &[(u8, u8)]) -> u32 {

    // transform pairs into collection of moves (opponent's, my)
    let moves = pairs.iter().map(|(first, second)| (first - A + 1, second - X + 1));

    return moves.map(|(opponent_move, my_move)| {
        game_points(opponent_move, my_move) + my_move as u32
    })
    .sum();
}

const LOSE: u8 = b'X';
const WIN: u8 = b'Z';

fn choose_my_move(opponent_move: u8, expected_result: u8) -> u8 {
    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    return match expected_result {
        LOSE => (opponent_move + 1) % 3 + 1,
        WIN => (opponent_move) % 3 + 1,
        _ => opponent_move,
    };
}

fn second_half(pairs: &[(u8, u8)]) -> u32 {
    let mut points: u32 = 0;

    for (first, second) in pairs {
        let opponent_move: u8 = first - A + 1;
        let my_move: u8 = choose_my_move(opponent_move, *second);

        let round_pts = game_points(opponent_move, my_move);
        points += round_pts;
        points += my_move as u32;
    }

    return points;
}

fn load(content: &str) -> Vec<(u8, u8)> {
    return content
        .lines()
        .map(|line| { let bytes = line.as_bytes(); (bytes[0], bytes[2]) })
        .collect();
}

fn main() -> std::io::Result<()> {
    let content = utils::load_data()?;
    let pairs: Vec<(u8, u8)> = load(&content);

    let pts1: u32 = first_half(&pairs);
    println!("1) Total points {}", pts1);

    let pts2: u32 = second_half(&pairs);
    println!("2) Total points {}", pts2);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_first_half() {
        let pairs = load("A Y\nB X\nC Z");
        let points: u32 = first_half(&pairs);
        assert_eq!(points, 15)
    }

    #[test]
    fn test_second_half() {
        let pairs = load("A Y\nB X\nC Z");
        let points: u32 = second_half(&pairs);
        assert_eq!(points, 12)
    }

    #[test]
    fn test_choose_my_move_lose() {
        assert_eq!(choose_my_move(ROCK, LOSE), SCISSORS, "For LOSE against ROCK, expected SCISSORS");
        assert_eq!(choose_my_move(PAPER, LOSE), ROCK, "For LOSE against PAPER, expected ROCK");
        assert_eq!(choose_my_move(SCISSORS, LOSE), PAPER, "For LOSE against SCISSORS, expected PAPER");
    }

    #[test]
    fn test_choose_my_move_win() {
        assert_eq!(choose_my_move(ROCK, WIN), PAPER, "For WIN against ROCK, expected PAPER");
        assert_eq!(choose_my_move(PAPER, WIN), SCISSORS, "For WIN against PAPER, expected SCISSORS");
        assert_eq!(choose_my_move(SCISSORS, WIN), ROCK, "For WIN against SCISSORS, expected ROCK");
    }

    #[test]
    fn test_choose_my_move_draw() {

        const DRAW: u8 = b'Y';

        assert_eq!(choose_my_move(ROCK, DRAW), ROCK);
        assert_eq!(choose_my_move(PAPER, DRAW), PAPER);
        assert_eq!(choose_my_move(SCISSORS, DRAW), SCISSORS);
    }

}
