pub fn most_calories() -> usize {
    elves()
        .iter()
        .map(|x| x.iter().sum())
        .max()
        .unwrap()
}

pub fn top_three_calories() -> usize {
    let mut top_three = elves()
        .iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<usize>>();
    top_three.sort_by(|a, b| b.cmp(a));

    top_three[0..=2].iter().sum()
}

fn elves() -> Vec<Vec<usize>> {
    include_str!("../data/day01.txt")
        .split("\n\n")
        .map(string_to_calories)
        .collect()
}

fn string_to_calories(raw: &str) ->  Vec<usize> {
    raw.split('\n').map(|x| x.parse::<usize>().unwrap_or(0)).collect()
}
