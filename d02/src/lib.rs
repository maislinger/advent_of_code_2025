pub fn solve(input: &str) -> String {
    let mut s1 = 0;
    let mut s2 = 0;

    for (lower, upper) in iter_ranges(input) {
        let mut invalid = std::collections::BTreeSet::new();

        iter_invalid_reps(2)
            .take_while(move |p| *p <= upper)
            .skip_while(move |p| *p < lower)
            .map(|p| {
                invalid.insert(p);
            })
            .count();

        s1 += invalid.iter().sum::<u64>();

        let n_digits = (upper.checked_ilog10().unwrap_or(0) + 1) as u64;

        for rep in 3..=n_digits {
            if rep == 20 {
                let possibility = 11_111_111_111_111_111_111;
                if possibility >= lower && possibility <= upper {
                    invalid.insert(possibility);
                }
                continue;
            }
            iter_invalid_reps(rep)
                .take_while(move |p| *p <= upper)
                .skip_while(move |p| *p < lower)
                .map(|p| {
                    invalid.insert(p);
                })
                .count();
        }

        s2 += invalid.iter().sum::<u64>();
    }

    // println!("Debug check: s1 = {}, s11 = {}", s1, s11);
    // println!("Debug check: s2 = {}, s21 = {}", s2, s21);

    format!("d02/01 = {}, d02/02 = {}", s1, s2)
}

fn iter_invalid_reps(reps: u64) -> impl Iterator<Item = u64> {
    (1..).map(move |i| repeat_number(i, reps))
}

fn repeat_number(i: u64, reps: u64) -> u64 {
    let mut result = i;
    let n_digits = i.checked_ilog10().unwrap_or(0) + 1;
    for _ in 1..reps {
        result = result * 10u64.pow(n_digits) + i;
    }
    result
}

fn iter_ranges(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(','))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let splitted = s.split_once('-').unwrap();
            let left = splitted.0.parse::<u64>().unwrap();
            let right = splitted.1.parse::<u64>().unwrap();
            (left, right)
        })
}
