use std::collections::BTreeMap;

pub fn solve(input: &str) -> String {
    // let input = "
    //     .......S.......
    //     ...............
    //     .......^.......
    //     ...............
    //     ......^.^......
    //     ...............
    //     .....^.^.^.....
    //     ...............
    //     ....^.^...^....
    //     ...............
    //     ...^.^...^.^...
    //     ...............
    //     ..^...^.....^..
    //     ...............
    //     .^.^.^.^.^...^.
    //     ...............
    //     ";

    let mut positions: BTreeMap<i64, u64> = BTreeMap::new();
    let mut s1 = 0;

    for line in input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        let mut new_positions = BTreeMap::new();

        for (i, c) in (0i64..).zip(line.chars()) {
            match c {
                'S' => {
                    new_positions.insert(i, 1);
                }
                '^' => {
                    if positions.contains_key(&i) {
                        s1 += 1;
                        new_positions
                            .entry(i - 1)
                            .and_modify(|v| *v += positions[&i])
                            .or_insert(positions[&i]);
                        new_positions
                            .entry(i + 1)
                            .and_modify(|v| *v += positions[&i])
                            .or_insert(positions[&i]);
                    }
                }
                '.' => {
                    if positions.contains_key(&i) {
                        new_positions
                            .entry(i)
                            .and_modify(|v| *v += positions[&i])
                            .or_insert(positions[&i]);
                    }
                }
                _ => panic!("Invalid input {}", c),
            }
        }

        positions = new_positions;
    }

    format!(
        "d07/01 = {}, d07/02 = {}",
        s1,
        positions.iter().map(|(_, &v)| v).sum::<u64>(),
    )
}
