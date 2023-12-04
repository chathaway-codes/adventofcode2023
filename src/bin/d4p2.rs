use std::collections::HashSet;
use std::io;

fn solve(g1: &[u32], g2: &[u32]) -> u32 {
    // Convert to hashmaps
    let mut winning_numbers = HashSet::new();
    for n in g1 {
        winning_numbers.insert(*n);
    }
    let mut score = 0;
    for n in g2 {
        if winning_numbers.contains(n) {
            score += 1;
        }
    }
    score
}

fn parse(input: &[String]) -> u32 {
    let mut card_counts = vec![1; input.len()];
    for (index, i) in input.iter().enumerate() {
        let i: Vec<&str> = i.split(':').collect();
        assert!(i.len() == 2);
        let games: Vec<&str> = i[1].split('|').collect();
        let g1: Vec<u32> = games[0]
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let g2: Vec<u32> = games[1]
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        // I won a copy of the next n cards for each copy of this I have
        let mut cards_won = solve(&g1, &g2) as usize;
        while cards_won > 0 {
            card_counts[index + cards_won] += card_counts[index];
            cards_won -= 1;
        }
    }
    card_counts.iter().sum()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lines().map(|s| s.unwrap()).collect();

    let count = parse(&lines);
    println!("{}", count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let input: Vec<String> = include_str!("d4p1.txt")
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        let res = parse(&input);
        assert_eq!(res, 30);
    }
}
