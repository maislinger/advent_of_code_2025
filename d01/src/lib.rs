pub fn solve(input: &str) -> String {
    let mut dial: i64 = 50;

    let mut count_zero = 0;
    let mut count_over_zero = 0;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let delta = parse_line(line);
        let previous_dial = dial;
        dial += delta;

        if dial % 100 == 0 {
            count_zero += 1;
        }

        count_over_zero += delta_over(dial, previous_dial);
    }

    format!("d01/01 = {}, d01/02 = {}", count_zero, count_over_zero)
}

fn parse_line(l: &str) -> i64 {
    let mut it = l.chars();
    let lr = it.next().expect("Day 01: Invalid Input");
    let amount_s = it.as_str();
    let amount: i64 = amount_s.parse().expect("Day 01: Failed to parse input");

    match lr {
        'L' => -amount,
        'R' => amount,
        _ => panic!("Day 01: Invalid Input"),
    }
}

fn delta_over(dial: i64, previous: i64) -> i64 {
    if dial == previous {
        return 0;
    }

    let delta = dial - previous;
    let delta_centum = (delta / 100).abs();

    let previous = previous.rem_euclid(100);
    let dial = previous + (delta % 100);

    let mut result = delta_centum;
    if (dial <= 0 || dial >= 100) && previous != 0 {
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta() {
        assert_eq!(delta_over(0, -50), 1);
        assert_eq!(delta_over(1, -1), 1);
        assert_eq!(delta_over(101, 99), 1);
        assert_eq!(delta_over(1, 0), 0);
        assert_eq!(delta_over(1, 100), 0);
        assert_eq!(delta_over(-10, 110), 2);
        assert_eq!(delta_over(101, 100), 0);
        assert_eq!(delta_over(200, 100), 1);
        assert_eq!(delta_over(0, 0), 0);
        assert_eq!(delta_over(0, 55), 1);
    }
}
