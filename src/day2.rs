use std::collections::HashMap;

pub fn tournament_total() -> usize {
    let score_table = HashMap::from([
          ("A X", 1 + 3),
          ("B X", 1 + 0),
          ("C X", 1 + 6),
          ("A Y", 2 + 6),
          ("B Y", 2 + 3),
          ("C Y", 2 + 0),
          ("A Z", 3 + 0),
          ("B Z", 3 + 6),
          ("C Z", 3 + 3),
    ]);

    scores(score_table)
}

pub fn correct_tournament_total() -> usize {
    let score_table = HashMap::from([
          ("A X", 3 + 0),
          ("B X", 1 + 0),
          ("C X", 2 + 0),
          ("A Y", 1 + 3),
          ("B Y", 2 + 3),
          ("C Y", 3 + 3),
          ("A Z", 2 + 6),
          ("B Z", 3 + 6),
          ("C Z", 1 + 6),
    ]);

    scores(score_table)
}

fn scores(score_table: HashMap<&str, usize>) -> usize {
    let data = include_str!("../data/day02.txt");

    let games: Vec<&str> = data
        .split('\n')
        .collect();

    games
        .iter()
        .map(|x| score_table.get(x).unwrap_or(&0))
        .sum()
}
