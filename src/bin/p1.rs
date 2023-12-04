use std::io;

fn solve(s: &str) -> u32 {
    let a = s
        .chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .unwrap();
    let b = s
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .unwrap();
    (a * 10) + b
}

fn parse(input: &[String]) -> u32 {
    let mut count = 0;
    for line in input {
        count += solve(line);
    }
    count
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
        let input: Vec<String> = include_str!("../example1.txt")
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        let res = parse(&input);
        assert!(res == 142);
    }
}
