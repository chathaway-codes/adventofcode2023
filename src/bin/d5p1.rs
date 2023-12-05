use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Range {
    src_start: u64,
    src_end: u64,
    dest_start: u64,
}

impl Range {
    fn contains(&self, target: u64) -> bool {
        if self.src_start <= target && self.src_end >= target {
            true
        } else {
            false
        }
    }

    fn remap(&self, target: u64) -> u64 {
        target + self.dest_start - self.src_start
    }
}

// to_ranges converts the inputs to a sorted slice of ranges
fn to_ranges(inputs: &[String]) -> Vec<Range> {
    let mut ranges = vec![];
    for s in inputs {
        let vals: Vec<u64> = s.split(' ').map(|s| s.parse().unwrap()).collect();
        ranges.push(Range {
            src_start: vals[1],
            src_end: vals[1] + vals[2] - 1,
            dest_start: vals[0],
        })
    }
    ranges.sort_by(|a, b| a.src_start.partial_cmp(&b.src_start).unwrap());
    ranges
}

fn find_next(target: u64, ranges: &[Range]) -> u64 {
    for v in ranges {
        if v.contains(target) {
            return v.remap(target);
        }
    }
    target
}

fn parse(input: &[String]) -> u64 {
    let seeds: Vec<&str> = input[0].split(':').collect();
    let seeds: Vec<u64> = seeds[1]
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    // Skip 0, and 1 which is a blank line
    let input = &input[2..];

    let mut maps: HashMap<&str, Vec<Range>> = HashMap::default();
    let mut i = 0;
    while i != input.len() {
        let name = &input[i];
        i += 1;
        let mut j = i;
        while j < input.len() && input[j].trim() != "" {
            j += 1;
        }
        let ranges = to_ranges(&input[i..j]);
        maps.insert(name, ranges);
        if j != input.len() && input[j].trim() == "" {
            j += 1;
        }
        i = j;
    }

    let order = vec![
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let mut lowest = u64::MAX;
    for seed in seeds {
        let mut cur = seed;
        for o in &order {
            let map = maps.get(*o).unwrap();
            cur = find_next(cur, map);
        }
        lowest = lowest.min(cur);
    }
    lowest
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
        let input: Vec<String> = include_str!("d5p1_sample.txt")
            .split("\n")
            .map(|s| String::from(s))
            .collect();
        let res = parse(&input);
        assert_eq!(res, 35);
    }

    #[test]
    fn ranges() {
        let r = Range {
            src_start: 50,
            src_end: 98,
            dest_start: 52,
        };
        assert_eq!(true, r.contains(79));
    }
}
