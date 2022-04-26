/// This struct receives the number of columns in the grid (length_x),
/// the number of rows (length_y) and a initial grid. After create the grid
/// you must use initialize_grid.

pub struct Grid {
    pub length_x: usize,
    pub length_y: usize,
    pub vector: Vec<Vec<u8>>,
}

impl Grid {
    /// This function generates a grid, it is composed by Vec of Vec of u8.
    pub fn initialize_grid(&mut self) {
        let vector1: Vec<Vec<u8>> = vec![vec![0; self.length_y]; self.length_x];
        self.vector = vector1;
    }

    /// This function returns the item in the row "row" and column "col".
    /// If it is an invalid index, it will return error.
    pub fn get(&mut self, row: usize, col: usize) -> u8 {
        self.vector[row][col]
    }

    /// This function puts the item "elem" in the row "row" and column "col".
    /// If it is an invalid index, it will return error.
    pub fn put(&mut self, elem: u8, row: usize, col: usize) {
        self.vector[row][col] = elem
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_initialize_rows_with_correct_length() {
        let length_x = 5;
        let length_y = 4;

        let mut grid = Grid {
            length_x: length_x,
            length_y: length_y,
            vector: Vec::new(),
        };
        grid.initialize_grid();

        assert_eq!(grid.vector.len(), 5)
    }

    #[test]
    fn grid_initialize_cols_with_correct_length() {
        let length_x = 5;
        let length_y = 4;

        let mut grid = Grid {
            length_x: length_x,
            length_y: length_y,
            vector: Vec::new(),
        };
        grid.initialize_grid();

        let mut is_ok = true;
        for i in 0..length_y {
            if grid.vector[i].len() != length_y {
                is_ok = false
            }
        }

        assert_eq!(is_ok, true)
    }

    #[test]
    fn get_works_correctly() {
        let length_x = 5;
        let length_y = 4;

        let mut grid = Grid {
            length_x: length_x,
            length_y: length_y,
            vector: Vec::new(),
        };
        grid.initialize_grid();

        grid.vector[3][2] = 4;

        assert_eq!(grid.get(3, 2), 4)
    }

    #[test]
    fn put_works_correctly() {
        let length_x = 5;
        let length_y = 4;

        let mut grid = Grid {
            length_x: length_x,
            length_y: length_y,
            vector: Vec::new(),
        };
        grid.initialize_grid();

        grid.put(9, 4, 1);

        assert_eq!(grid.vector[4][1], 9)
    }
}
