use std::fmt::Debug;
use std::io;

#[derive(Clone)]
struct Range {
    src_start: u64,
    src_end: u64,
    dest_start: u64,
}

impl Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}) => {}",
            self.src_start, self.src_end, self.dest_start
        )
    }
}

impl Range {
    fn contains(&self, target: u64) -> bool {
        if self.src_start <= target && self.src_end >= target {
            true
        } else {
            false
        }
    }

    // Converts any of other which into self to a new range offset appropriately
    fn subrange(&self, other: &Range) -> Option<Range> {
        if self.contains(other.src_start) && !self.contains(other.src_end) {
            // Include a subset from [other.src_start, self.src_end)
            Some(Range {
                src_start: self.remap(other.src_start),
                src_end: self.remap(self.src_end),
                dest_start: 0,
            })
        } else if self.contains(other.src_end) && !self.contains(other.src_start) {
            // Include a subset from [self.src_start, other.src_end)
            Some(Range {
                src_start: self.remap(self.src_start),
                src_end: self.remap(other.src_end),
                dest_start: 0,
            })
        } else if self.contains(other.src_start) && self.contains(other.src_end) {
            Some(Range {
                src_start: self.remap(other.src_start),
                src_end: self.remap(other.src_end),
                dest_start: 0,
            })
        } else {
            None
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
            src_end: vals[1] + vals[2],
            dest_start: vals[0],
        })
    }
    ranges.sort_by(|a, b| a.src_start.partial_cmp(&b.src_start).unwrap());
    // Fill in any empty segments
    let mut segments_to_add = vec![];
    // Check if we need to add first part
    if ranges[0].src_start != 0 {
        segments_to_add.push(Range {
            src_start: 0,
            src_end: ranges[0].src_start,
            dest_start: 0,
        })
    }

    for i in 1..ranges.len() {
        if ranges[i - 1].src_end != ranges[i].src_start {
            segments_to_add.push(Range {
                src_start: ranges[i - 1].src_end,
                src_end: ranges[i].src_start,
                dest_start: ranges[i - 1].src_end,
            });
        }
    }

    // Add an end
    let last = ranges[ranges.len() - 1].clone();
    segments_to_add.push(Range {
        src_start: last.src_end,
        src_end: u64::MAX,
        dest_start: last.src_end,
    });
    ranges.append(&mut segments_to_add);
    ranges.sort_by(|a, b| a.src_start.partial_cmp(&b.src_start).unwrap());

    ranges
}

fn parse(input: &[String]) -> u64 {
    let seeds: Vec<&str> = input[0].split(':').collect();
    let seeds: Vec<u64> = seeds[1]
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    // Get the list of seeds
    let mut seed_ranges = vec![];
    let mut i = 0;
    while i < seeds.len() {
        seed_ranges.push(Range {
            src_start: seeds[i],
            src_end: seeds[i] + seeds[i + 1],
            dest_start: 0,
        });
        i += 2;
    }
    // Skip 0, and 1 which is a blank line
    let input = &input[2..];

    // Get the other maps
    let mut maps: Vec<Vec<Range>> = vec![];
    let mut i = 0;
    while i != input.len() {
        i += 1;
        let mut j = i;
        while j < input.len() && input[j].trim() != "" {
            j += 1;
        }
        let ranges = to_ranges(&input[i..j]);
        maps.push(ranges);
        if j != input.len() && input[j].trim() == "" {
            j += 1;
        }
        i = j;
    }

    let mut lowest = u64::MAX;
    for seed in seed_ranges {
        let mut possible_ranges = vec![seed];
        for map in &maps {
            // Ok, so we know with confidence that each range will fit entirely within
            // the superset.
            let mut next_ranges = vec![];
            for possibility in &possible_ranges {
                for r in map {
                    if let Some(r) = r.subrange(possibility) {
                        next_ranges.push(r);
                    }
                }
            }
            possible_ranges = next_ranges;
        }
        for r in &possible_ranges {
            lowest = lowest.min(r.src_start);
        }
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
        assert_eq!(res, 46);
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
