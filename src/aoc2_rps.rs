/*
 * Challenge:
 * ABC and XYZ corresponds to Rock, Paper, and Scissors respectively.
 * A strategy guide is laid out like this:
 * D W
 * D W
 * D W
 * ...
 * where D is any of ABC and W is any of XYZ.
 * Playing Rock will give a score of 1, Paper gives a score of 2, Scissors a score of 3.
 * Winning will grant you 6 points, losing grants you 0 points, and drawing gives 3 points.
 * If D is the opponent and W is you, output the total score for every round in the strategy guide.
 * 
 * 2nd part:
 * XYZ means the round needs to end in a loss, draw, or win respectively.
 * Output the total score.
 */

#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors
}
enum ShouldWin {
    Lose,
    Draw,
    Win
}

pub fn rps(part_2: bool) {
    let strategy_guide: Vec<&str> =
    include_str!("assets/input_day2.txt")
        .split("\n")
        .map(|s|s.trim())
        .collect();
    let mut total_score: usize = 0;
    for i in strategy_guide {
        let round: Vec<&str> = i.split(' ').collect();
        if round.len() < 2 {
            continue;
        }
        let opponent_play: RPS = match round[0] {
            "A" => {RPS::Rock}
            "B" => {RPS::Paper}
            "C" => {RPS::Scissors}
            _ => {
                println!("Opponent RPS `{}` not valid!", round[0]);
                continue;
            }
        };
        let player_play: RPS = if part_2 {
            let win_state = match round[1] {
                "X" => {ShouldWin::Lose}
                "Y" => {ShouldWin::Draw}
                "Z" => {ShouldWin::Win}
                _ => {
                    println!("Player RPS `{}` not valid!", round[1]);
                    continue;
                }
            };
            match win_state {
                ShouldWin::Lose => {
                    match opponent_play {
                        RPS::Rock => RPS::Scissors,
                        RPS::Paper => RPS::Rock,
                        RPS::Scissors => RPS::Paper
                    }
                }
                ShouldWin::Draw => opponent_play,
                ShouldWin::Win => {
                    match opponent_play {
                        RPS::Rock => RPS::Paper,
                        RPS::Paper => RPS::Scissors,
                        RPS::Scissors => RPS::Rock
                    }
                }
            }
        } else {
            match round[1] {
                "X" => {RPS::Rock}
                "Y" => {RPS::Paper}
                "Z" => {RPS::Scissors}
                _ => {
                    println!("Player RPS `{}` not valid!", round[1]);
                    continue;
                }
            }
        };

        let mut score: usize = match player_play {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3
        };

        let win_score = match (opponent_play, player_play) {
            // Draw (3 pts)
            (RPS::Rock, RPS::Rock)
            |(RPS::Paper, RPS::Paper)
            |(RPS::Scissors, RPS::Scissors) => 3,

            // Lose (0 pts)
            (RPS::Rock, RPS::Scissors)
            |(RPS::Paper, RPS::Rock)
            |(RPS::Scissors, RPS::Paper) => 0,

            // Win (6 pts)
            (RPS::Rock, RPS::Paper)
            |(RPS::Paper, RPS::Scissors)
            |(RPS::Scissors, RPS::Rock) => 6,
        };
        score += win_score;
        total_score += score;
    }
    println!("Total score (P{}): {total_score}", part_2 as usize + 1);
}