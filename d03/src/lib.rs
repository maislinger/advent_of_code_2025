pub fn solve(input: &str) -> String {
    let mut s1 = 0;
    let mut s2 = 0;

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let (delta_1, delta_2) = max_joltage(line);
        s1 += delta_1;
        s2 += delta_2;
    }

    format!("d03/01 = {}, d03/02 = {}", s1, s2)
}

fn max_joltage(line: &str) -> (u64, u64) {
    let line = line.trim();
    let mut state_machine = StateMashine::<2>::new();
    let mut state_machine_2 = StateMashine::<12>::new();

    for c in line.chars() {
        let n = c.to_digit(10).unwrap() as u8;
        state_machine.add(n);
        state_machine_2.add(n);
    }

    (state_machine.current_max(), state_machine_2.current_max())
}

#[derive(Debug)]
struct StateMashine<const N: usize> {
    vals: [u8; N],
}

impl<const N: usize> StateMashine<N> {
    fn new() -> Self {
        Self { vals: [0; N] }
    }

    fn add(&mut self, new_number: u8) {
        for i in 1..N {
            if self.vals[i] > self.vals[i - 1] {
                self.shift_vals_at(i - 1);
                self.vals[N - 1] = new_number;
                return;
            }
        }

        if new_number > self.vals[N - 1] {
            self.vals[N - 1] = new_number;
        }
    }

    fn shift_vals_at(&mut self, index: usize) {
        for i in index..(N - 1) {
            self.vals[i] = self.vals[i + 1];
        }
    }

    fn current_max(&self) -> u64 {
        self.vals
            .iter()
            .fold(0u64, |acc, new_item| 10 * acc + (*new_item as u64))
    }
}
