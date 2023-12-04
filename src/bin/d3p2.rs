use std::collections::HashSet;
use std::io;

fn find_number(row: &Vec<char>, i: usize) -> (u32, usize) {
    assert!(row[i] != '.');
    let (mut low, mut high) = (i, i);
    while low > 0 && row[low].is_digit(10) {
        low -= 1;
    }
    if !row[low].is_digit(10) {
        low += 1;
    }
    while high < row.len() && row[high].is_digit(10) {
        high += 1;
    }

    (String::from_iter(&row[low..high]).parse().unwrap(), low)
}

fn solve(s: &[Vec<char>]) -> u32 {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut total = 0;
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            if s[i][j] == '*' && !s[i][j].is_digit(10) {
                let mut m: i32 = 0;
                let mut local_total = 1;
                // Check the positions
                if j > 0 && s[i][j - 1].is_digit(10) {
                    let (x, y) = find_number(&s[i], j - 1);
                    let y = (i, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if j + 1 < s[i].len() && s[i][j + 1].is_digit(10) {
                    let (x, y) = find_number(&s[i], j + 1);
                    let y = (i, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if j > 0 && i > 0 && s[i - 1][j - 1].is_digit(10) {
                    let (x, y) = find_number(&s[i - 1], j - 1);
                    let y = (i - 1, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if j + 1 < s[i].len() && i + 1 < s.len() && s[i + 1][j + 1].is_digit(10) {
                    let (x, y) = find_number(&s[i + 1], j + 1);
                    let y = (i + 1, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if j > 0 && i + 1 < s.len() && s[i + 1][j - 1].is_digit(10) {
                    let (x, y) = find_number(&s[i + 1], j - 1);
                    let y = (i + 1, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if j + 1 < s[i].len() && i > 0 && s[i - 1][j + 1].is_digit(10) {
                    let (x, y) = find_number(&s[i - 1], j + 1);
                    let y = (i - 1, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if i > 0 && s[i - 1][j].is_digit(10) {
                    let (x, y) = find_number(&s[i - 1], j);
                    let y = (i - 1, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if i + 1 < s.len() && s[i + 1][j].is_digit(10) {
                    let (x, y) = find_number(&s[i + 1], j);
                    let y = (i + 1, y);
                    if !seen.contains(&y) {
                        m += 1;
                        local_total *= x;
                        seen.insert(y);
                    }
                }
                if m == 2 {
                    total += local_total;
                }
            }
        }
    }
    total
}

fn parse(input: &[String]) -> u32 {
    //create a grid of chars
    let input: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();
    solve(&input)
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
        let input: Vec<String> = include_str!("d3p1.txt")
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        let res = parse(&input);
        assert_eq!(res, 467835);
    }
}
