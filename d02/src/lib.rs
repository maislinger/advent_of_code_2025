use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

pub fn solve(input: &str) -> String {
    let ranges: Vec<_> = iter_ranges(input).collect();
    let (s1, s2) = ranges
        .par_iter()
        .flat_map(|&(l, u)| (l..u).into_par_iter())
        .map(|n| {
            let buffer = split_number(n);
            let invalid_level = buffer_invalid_level(&buffer);
            match invalid_level {
                1 => (n, n),
                2 => (0, n),
                _ => (0, 0),
            }
        })
        .reduce(
            || (0, 0),
            |sums, element| (sums.0 + element.0, sums.1 + element.1),
        );

    format!("d02/01 = {}, d02/02 = {}", s1, s2)
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

fn split_number(mut n: u64) -> Vec<u8> {
    let mut buffer = Vec::new();

    while n > 0 {
        buffer.push((n % 10) as u8);
        n /= 10;
    }

    buffer
}

fn buffer_invalid_level(buffer: &[u8]) -> u64 {
    let len = buffer.len();

    for sublen in (1..=(len / 2)).rev() {
        if !len.is_multiple_of(sublen) {
            continue;
        }

        let iter_1 = buffer.chunks_exact(sublen);
        let iter_2 = buffer.chunks_exact(sublen).skip(1);

        let mut all_equal = true;
        for (chunk_1, chunk_2) in iter_1.zip(iter_2) {
            if chunk_1 != chunk_2 {
                all_equal = false;
                break;
            }
        }

        if all_equal && len.is_multiple_of(2) && sublen == len / 2 {
            return 1;
        } else if all_equal {
            return 2;
        }
    }

    0
}
