use std::io;

fn solve(s: &str) -> u32 {
    let subs = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let chars: Vec<char> = s.chars().collect();
    let (mut first, mut last) = (None, None);
    for (i, _) in (&chars).iter().enumerate() {
        for sub in &subs {
            if i + sub.0.len() <= chars.len()
                && sub.0 == String::from_iter(&chars[i..i + sub.0.len()])
            {
                if first.is_none() {
                    first = Some(sub.1)
                }
                last = Some(sub.1)
            }
        }
        if let Some(d) = chars[i].to_digit(10) {
            if first.is_none() {
                first = Some(d);
            }
            last = Some(d);
        }
    }
    (first.unwrap() * 10) + last.unwrap()
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
        assert_eq!(res, 142);
    }
    #[test]
    fn example2() {
        let input: Vec<String> = include_str!("../example2.txt")
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        let res = parse(&input);
        assert_eq!(res, 281);
    }
}
