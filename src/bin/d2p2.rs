use std::collections::HashMap;
use std::io;

use regex::Regex;

fn solve(s: &str, r: &Regex) -> u32 {
    let mut max: HashMap<&str, u32> = HashMap::default();
    for game in s.split(";").into_iter() {
        for (_, [count, color]) in r.captures_iter(game).map(|c| c.extract()) {
            let v = count.parse().unwrap();
            max.entry(&color)
                .and_modify(|e| {
                    if *e < v {
                        *e = v;
                    }
                })
                .or_insert(v);
        }
    }
    max["red"] * max["green"] * max["blue"]
}

fn parse(input: &[String]) -> u32 {
    let r = Regex::new(r#"(\d+) (red|blue|green)"#).unwrap();
    let mut count = 0;
    for line in input {
        count += solve(line, &r);
    }
    count as u32
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
        let input: Vec<String> = include_str!("d2p1_sample.txt")
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        let res = parse(&input);
        assert_eq!(res, 2286);
    }
}
