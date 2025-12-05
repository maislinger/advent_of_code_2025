pub fn solve(input: &str) -> String {
    let mut line_iter = input.trim().lines().map(|l| l.trim());

    let mut ranges = Vec::new();

    for line in line_iter.by_ref() {
        if line.is_empty() {
            break;
        }

        let (left, right) = line.split_once('-').unwrap();
        let lower = left.parse::<u64>().unwrap();
        let upper = right.parse::<u64>().unwrap();

        ranges = add_range(ranges, (lower, upper));
    }

    let mut count_valid = 0;
    for line in line_iter {
        if line.is_empty() {
            continue;
        }

        let id = line.parse::<u64>().unwrap();

        let valid = ranges.iter().any(|&(l, u)| l <= id && id <= u);
        if valid {
            count_valid += 1;
        }
    }

    let count_all = ranges.iter().map(|(l, u)| u - l + 1).sum::<u64>();

    format!("d05/01 = {}, d05/02 = {}", count_valid, count_all)
}

fn add_range(ranges: Vec<(u64, u64)>, mut new_range: (u64, u64)) -> Vec<(u64, u64)> {
    let mut new_ranges = Vec::new();

    let mut added = false;
    for range in ranges {
        if range.1 < new_range.0 {
            new_ranges.push(range);
        } else if range.0 > new_range.1 {
            if !added {
                new_ranges.push(new_range);
                added = true;
            }
            new_ranges.push(range);
        } else if range.0 > new_range.0 && range.1 < new_range.1 {
            // do nothing
        } else {
            new_range.0 = new_range.0.min(range.0);
            new_range.1 = new_range.1.max(range.1);
        }
    }

    if !added {
        new_ranges.push(new_range);
    }

    new_ranges
}
