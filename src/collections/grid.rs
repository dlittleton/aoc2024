#[derive(Debug, Clone)]
pub struct Grid<T> {
    values: Vec<Vec<T>>,
}

impl<T, I> FromIterator<I> for Grid<T>
where
    I: Iterator<Item = T>,
{
    fn from_iter<B: IntoIterator<Item = I>>(items: B) -> Self {
        let values: Vec<Vec<_>> = items.into_iter().map(|item| item.collect()).collect();

        let length = values[0].len();
        if !values.iter().all(|v| v.len() == length) {
            panic!(
                "All rows must have the same number of columns. Expected count {}",
                length
            );
        }

        Grid { values }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, i: usize, j: usize) -> &T {
        &self.values[i][j]
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.values[i][j]
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.values
            .iter()
            .enumerate()
            .flat_map(|(i, v)| v.iter().enumerate().map(move |(j, c)| (i, j, c)))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.values
            .iter_mut()
            .enumerate()
            .flat_map(|(i, v)| v.iter_mut().enumerate().map(move |(j, c)| (i, j, c)))
    }

    pub fn rows(&self) -> usize {
        self.values.len()
    }

    pub fn cols(&self) -> usize {
        self.values[0].len()
    }

    pub fn row_wise_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        self.values.iter().map(|v| v.iter())
    }

    pub fn col_wise_iter(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (0..self.cols()).map(move |c| (0..self.rows()).map(move |r| self.get(r, c)))
    }

    pub fn insert_row_at(&mut self, i: usize, values: impl IntoIterator<Item = T>) {
        let new_row: Vec<_> = values.into_iter().take(self.cols()).collect();

        if new_row.len() != self.cols() {
            panic!(
                "Not enough elements. Expected {}, Found {}",
                self.cols(),
                new_row.len()
            )
        }

        self.values.insert(i, new_row);
    }

    pub fn insert_col_at(&mut self, i: usize, values: impl IntoIterator<Item = T>) {
        let mut new_col: Vec<_> = values.into_iter().take(self.rows()).collect();

        if new_col.len() != self.rows() {
            panic!(
                "Not enough elements. Expected {}, Found {}",
                self.rows(),
                new_col.len()
            )
        }

        self.values
            .iter_mut()
            .rev()
            .for_each(|v| v.insert(i, new_col.pop().unwrap()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_grid() -> Grid<char> {
        let lines = vec!["abc", "def", "ghi", "jkl"];
        lines.iter().map(|l| l.chars()).collect()
    }

    #[test]
    fn test_direct_values() {
        let grid = create_test_grid();
        assert!(!grid.values.is_empty());
        assert_eq!(grid.values.len(), 4);
        assert_eq!(grid.values[0].len(), 3);
        assert_eq!(grid.values[0][0], 'a');
        assert_eq!(grid.values[0][1], 'b');
        assert_eq!(grid.values[1][1], 'e');
    }

    #[test]
    fn test_get_by_index() {
        let grid = create_test_grid();
        assert_eq!(*grid.get(1, 1), 'e');
    }

    #[test]
    fn test_enumerate() {
        let grid = create_test_grid();
        let positions: Vec<_> = grid.enumerate().map(|(a, b, c)| (a, b, *c)).collect();

        assert_eq!(positions[0], (0, 0, 'a'));
        assert_eq!(positions[1], (0, 1, 'b'));
        assert_eq!(positions[4], (1, 1, 'e'));
    }

    #[test]
    fn test_rows() {
        let grid = create_test_grid();
        assert_eq!(grid.rows(), 4);
    }

    #[test]
    fn test_cols() {
        let grid = create_test_grid();
        assert_eq!(grid.cols(), 3);
    }

    #[test]
    #[should_panic]
    fn test_invalid_collect() {
        let lines = vec!["abc", "defg"];
        let _: Grid<_> = lines.iter().map(|l| l.chars()).collect();
    }

    #[test]
    fn test_row_wise_iter() {
        let grid = create_test_grid();
        let row: Vec<_> = grid.row_wise_iter().next().unwrap().map(|c| *c).collect();

        assert_eq!(row, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_col_wise_iter() {
        let grid = create_test_grid();
        let col: Vec<_> = grid.col_wise_iter().next().unwrap().map(|c| *c).collect();

        assert_eq!(col, vec!['a', 'd', 'g', 'j']);
    }

    #[test]
    fn test_insert_row_at() {
        let mut grid = create_test_grid();
        grid.insert_row_at(1, "123".chars());

        assert_eq!(*grid.get(1, 0), '1');
    }

    #[test]
    fn test_insert_col_at() {
        let mut grid = create_test_grid();
        grid.insert_col_at(1, "1234".chars());

        assert_eq!(*grid.get(0, 1), '1');
    }
}
