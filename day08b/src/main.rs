use std::cmp::max;

fn main() {
    let values = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|letter| letter.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut grid = Grid::new(values);

    // grid.process_rows();
    // grid.process_cols();
    // grid.process_rows_rev();
    // grid.process_cols_rev();
    dbg!(&grid.visible);

    println!("max_scenic_score: {}", grid.max_scenic_score());
}

struct Grid {
    values: Vec<Vec<i32>>,
    visible: Vec<Vec<i32>>,
    rows: usize,
    cols: usize,
}

struct GridCell {
    row: usize,
    col: usize,
    val: i32,
}
struct MonotonicStack(Vec<GridCell>);

impl MonotonicStack {
    fn push(&mut self, value: i32) {
        while let Some(_) = self.0.last().and_then(|top| Some(value > top.val)) {
            self.0.pop();
        }
    }
}

impl Grid {
    fn new(values: Vec<Vec<i32>>) -> Self {
        let (rows, cols) = (values.len(), values[0].len());
        let visible = vec![vec![1; cols]; rows];
        Grid {
            values,
            visible,
            rows,
            cols,
        }
    }

    #[inline]
    fn is_border(&self, row: usize, col: usize) -> bool {
        col == 0 || row == 0 || col == self.cols - 1 || row == self.rows - 1
    }

    #[inline]
    fn update_visibility(
        &mut self,
        current_max: &mut i32,
        max_row: &mut usize,
        max_col: &mut usize,
        row: usize,
        col: usize,
    ) {
        if self.is_border(row, col) {
            self.visible[row][col] = 0
        }
        self.visible[row][col] *= ((row + col - *max_row - *max_col) as i32).abs();
        if self.values[row][col] >= *current_max {
            println!(
                "{row} - {max_row} | {col} - {max_col} | {} | {}",
                self.visible[row][col], self.visible[row][col]
            );

            // println!("({row}|{col}) -> {} > {current_max}", self.values[row][col]);
            // println!(
            //     "{}|{}|{}|{} -> {}",
            //     row, col, *max_row, *max_col, self.visible[row][col]
            // );
            (*max_row, *max_col, *current_max) = (row, col, self.values[row][col]);
        }
    }

    fn max_scenic_score(&self) -> i32 {
        //ignore edges1
        (1..self.rows - 1)
            .flat_map(move |row| (1..self.cols - 1).clone().map(move |col| [row, col]))
            .map(|[row, col]| self.visible[row][col])
            .max()
            .unwrap()
        // *self.visible.iter().flat_map(|x| x.iter()).max().unwrap()
    }

    // fn process_rows(&mut self) {
    //     for row in 0..self.rows {
    //         let (mut current_max, mut max_row, mut max_col) = (-1, 0, 0);
    //         // print!("NEW ROW");
    //         for col in 0..self.cols {
    //             self.update_visibility(&mut current_max, &mut max_row, &mut max_col, row, col)
    //         }
    //     }
    // }

    fn process_cols(&mut self) {
        for col in 0..self.cols {
            let (mut current_max, mut max_row, mut max_col) = (-1, 0, 0);
            for row in 0..self.rows {
                self.update_visibility(&mut current_max, &mut max_row, &mut max_col, row, col)
            }
        }
    }

    // fn process_rows_rev(&mut self) {
    //     for row in 0..self.rows {
    //         let (mut current_max, mut max_row, mut max_col) = (-1, 0, 0);
    //         for col in (0..self.cols).rev() {
    //             self.update_visibility(&mut current_max, &mut max_row, &mut max_col, row, col)
    //         }
    //     }
    // }

    // fn process_cols_rev(&mut self) {
    //     for col in 0..self.cols {
    //         let (mut current_max, mut max_row, mut max_col) = (-1, 0, 0);
    //         for row in (0..self.rows).rev() {
    //             self.update_visibility(&mut current_max, &mut max_row, &mut max_col, row, col)
    //         }
    //     }
    // }
}

// fn iterate
