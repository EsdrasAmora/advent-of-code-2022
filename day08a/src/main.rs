use std::cmp::max;

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
    // dbg!(&grid.visible);

    println!("visible: {}", grid.count_visible());
}

struct Grid {
    values: Vec<Vec<u32>>,
    visible: Vec<Vec<bool>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(values: Vec<Vec<u32>>) -> Self {
        let (rows, cols) = (values.len(), values[0].len());
        let visible = vec![vec![false; cols]; rows];
        Grid {
            values,
            visible,
            rows,
            cols,
        }
    }

    #[allow(dead_code)]
    fn columns(&self) -> impl Iterator<Item = &u32> {
        (0..self.cols)
            .flat_map(move |col| (0..self.rows).clone().map(move |row| [row, col]))
            .map(|[row, col]| &self.values[row][col])
    }

    #[inline]
    fn is_border(&self, row: usize, col: usize) -> bool {
        col == 0 || row == 0 || col == self.cols - 1 || row == self.rows - 1
    }

    #[inline]
    fn update_visibility(&mut self, current_max: &mut u32, row: usize, col: usize) {
        if self.is_border(row, col) {
            self.visible[row][col] = true;
            *current_max = max(*current_max, self.values[row][col]);
            return;
        }
        if self.values[row][col] > *current_max {
            self.visible[row][col] = true;
            *current_max = self.values[row][col];
        }
    }

    fn count_visible(&self) -> u32 {
        self.visible
            .iter()
            .flat_map(|x| x.iter().map(|y| if *y { 1 } else { 0 }))
            .sum()
    }

    fn process_rows(&mut self) {
        for row in 0..self.rows {
            let mut current_max = 0;
            for col in 0..self.cols {
                self.update_visibility(&mut current_max, row, col)
            }
        }
    }

    fn process_cols(&mut self) {
        for col in 0..self.cols {
            let mut current_max = 0;
            for row in 0..self.rows {
                self.update_visibility(&mut current_max, row, col)
            }
        }
    }

    fn process_rows_rev(&mut self) {
        for row in 0..self.rows {
            let mut current_max = 0;
            for col in (0..self.cols).rev() {
                self.update_visibility(&mut current_max, row, col)
            }
        }
    }

    fn process_cols_rev(&mut self) {
        for col in 0..self.cols {
            let mut current_max = 0;
            for row in (0..self.rows).rev() {
                self.update_visibility(&mut current_max, row, col)
            }
        }
    }
}

// fn iterate
