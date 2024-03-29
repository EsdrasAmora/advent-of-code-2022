fn main() {
    let values = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|letter| letter.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grid = Grid::new(values);

    grid.process_rows();
    grid.process_cols();
    grid.process_rows_rev();
    grid.process_cols_rev();
    dbg!(&grid.visible);

    println!("max_scenic_score: {}", grid.max_scenic_score());
}

struct Grid {
    values: Vec<Vec<u32>>,
    visible: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct GridCell {
    row: usize,
    col: usize,
    val: u32,
}

#[derive(Debug)]
struct MonotonicStack(Vec<GridCell>);

impl Grid {
    fn new(values: Vec<Vec<u32>>) -> Self {
        let (rows, cols) = (values.len(), values[0].len());
        let visible = vec![vec![1; cols]; rows];
        Grid {
            values,
            visible,
            rows,
            cols,
        }
    }

    fn max_scenic_score(&self) -> u32 {
        (1..self.rows - 1)
            .flat_map(move |row| (1..self.cols - 1).clone().map(move |col| [row, col]))
            .map(|[row, col]| self.visible[row][col])
            .max()
            .unwrap()
    }

    fn process_rows(&mut self) {
        let mut initial = GridCell::default();
        let mut stack = MonotonicStack(Vec::with_capacity(self.cols));

        for row in 0..self.rows {
            stack.clear();
            initial.row = row;
            for col in 0..self.cols {
                let current = GridCell {
                    col,
                    row,
                    val: self.values[row][col],
                };
                self.update_visibility(&mut stack, current, &initial)
            }
        }
    }

    fn process_cols(&mut self) {
        let mut initial = GridCell::default();
        let mut stack = MonotonicStack(Vec::with_capacity(self.rows));

        for col in 0..self.cols {
            stack.clear();
            initial.col = col;
            for row in 0..self.rows {
                let current = GridCell {
                    col,
                    row,
                    val: self.values[row][col],
                };
                self.update_visibility(&mut stack, current, &initial)
            }
        }
    }

    fn process_rows_rev(&mut self) {
        let mut stack = MonotonicStack(Vec::with_capacity(self.cols));
        let mut start = GridCell {
            row: self.rows - 1,
            col: self.cols - 1,
            val: 0,
        };

        for row in 0..self.rows {
            stack.clear();
            start.row = row;
            for col in (0..self.cols).rev() {
                let current = GridCell {
                    col,
                    row,
                    val: self.values[row][col],
                };
                self.update_visibility(&mut stack, current, &start)
            }
        }
    }

    fn process_cols_rev(&mut self) {
        let mut stack = MonotonicStack(Vec::with_capacity(self.rows));
        let mut initial = GridCell {
            row: self.rows - 1,
            col: self.cols - 1,
            val: 0,
        };

        for col in 0..self.cols {
            stack.clear();
            initial.col = col;
            for row in (0..self.rows).rev() {
                let current = GridCell {
                    col,
                    row,
                    val: self.values[row][col],
                };
                self.update_visibility(&mut stack, current, &initial)
            }
        }
    }

    #[inline]
    fn update_visibility(
        &mut self,
        stack: &mut MonotonicStack,
        current: GridCell,
        start: &GridCell,
    ) {
        stack.push(current.clone());
        let previous = stack.previous(start);

        self.visible[current.row][current.col] *=
            (current.row as i32 + current.col as i32 - previous.row as i32 - previous.col as i32)
                .abs() as u32;
    }
}

impl MonotonicStack {
    #[inline]
    fn push(&mut self, cell: GridCell) {
        while let Some(_) = self
            .0
            .last()
            .and_then(|top| (cell.val > top.val).then_some(true))
        {
            self.0.pop();
        }

        self.0.push(cell);
    }

    #[inline]
    fn previous(&self, default: &GridCell) -> GridCell {
        if self.0.len() == 1 {
            return default.clone();
        }
        return self.0[self.0.len() - 2].clone();
    }

    #[inline]
    fn clear(&mut self) {
        self.0.clear();
    }
}
