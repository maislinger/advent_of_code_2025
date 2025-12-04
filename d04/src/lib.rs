pub fn solve(input: &str) -> String {
    let mut warehouse = Warehouse::from_input(input);
    let first_remove = warehouse.remove_accessible_paper();

    let mut total_removed = first_remove;
    loop {
        let removed = warehouse.remove_accessible_paper();
        if removed == 0 {
            break;
        }
        total_removed += removed;
    }

    format!("d04/01 = {}, d04/02 = {}", first_remove, total_removed)
}

#[derive(Debug)]
struct Warehouse {
    fields: Vec<FieldState>,
    width: usize,
    height: usize,
    indices_paper: Vec<usize>,
}

impl Warehouse {
    fn from_input(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut fields = Vec::new();
        let mut indices_paper = Vec::new();
        let mut next_index = 0;
        for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
            height += 1;
            let mut local_width = 0;
            for c in line.chars() {
                local_width += 1;
                match c {
                    '.' => fields.push(FieldState::Empty),
                    '@' => {
                        fields.push(FieldState::Paper);
                        indices_paper.push(next_index);
                    }
                    _ => panic!("Invalid character in input: {}", c),
                }
                next_index += 1;
            }

            if width == 0 {
                width = local_width;
            } else if width != local_width {
                panic!("Inconsistent line widths in input");
            }
        }

        Self {
            fields,
            width,
            height,
            indices_paper,
        }
    }

    fn remove_accessible_paper(&mut self) -> usize {
        let mut to_remove = Vec::new();
        let mut new_indices_paper = Vec::new();

        for &index in self.indices_paper.iter() {
            if !self.fields[index].is_paper() {
                continue;
            }

            let neighbor_count = self
                .iter_neighbor_indices(index)
                .filter(|&ni| self.fields[ni].is_paper())
                .count();

            if neighbor_count < 4 {
                to_remove.push(index);
            } else {
                new_indices_paper.push(index);
            }
        }

        for index in to_remove.iter() {
            self.fields[*index] = FieldState::Empty;
        }

        self.indices_paper = new_indices_paper;

        to_remove.len()
    }

    fn iter_neighbor_indices(&self, index: usize) -> impl Iterator<Item = usize> {
        iter_neighbors_indices(index, self.width, self.height)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldState {
    Empty,
    Paper,
}

impl FieldState {
    fn is_paper(&self) -> bool {
        *self == FieldState::Paper
    }
}

fn iter_neighbors_indices(
    index: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = usize> {
    let i = (index / width) as i64;
    let j = (index % width) as i64;

    [
        (0i64, 1i64),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(move |(di, dj)| {
        let ni = i + di;
        let nj = j + dj;
        if ni >= 0 && ni < (height as i64) && nj >= 0 && nj < (width as i64) {
            Some((ni as usize) * width + (nj as usize))
        } else {
            None
        }
    })
}
