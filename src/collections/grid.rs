use std::iter::repeat_n;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    values: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

lazy_static! {
    pub static ref CARDINAL_DIRECTIONS: Vec<Direction> = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West
    ];
}

pub fn get_direction_delta(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    }
}

#[derive(Clone, Copy)]
pub struct Position<'a, T> {
    row: usize,
    col: usize,
    grid: &'a Grid<T>,
}

impl<'a, T> Position<'a, T> {
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn value(&self) -> &T {
        self.grid.get(self.row, self.col)
    }

    pub fn get_neighbor(&self, dir: Direction) -> Option<Self> {
        let (dr, dc) = get_direction_delta(dir);
        self.grid
            .position(self.row as isize + dr, self.col as isize + dc)
    }

    pub fn get_neighbors(
        &self,
        dirs: &'static [Direction],
    ) -> impl Iterator<Item = Self> + use<'a, '_, T> {
        dirs.iter().filter_map(|d| self.get_neighbor(*d))
    }
}

impl<T> From<Position<'_, T>> for (usize, usize) {
    fn from(val: Position<'_, T>) -> Self {
        (val.row, val.col)
    }
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

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(rows: usize, cols: usize, init: T) -> Self {
        (0..rows).map(|_| repeat_n(init, cols)).collect()
    }
}

impl<T> Grid<T> {
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.values[row][col]
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.values[row][col]
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.values.iter().enumerate().flat_map(|(row, v)| {
            v.iter()
                .enumerate()
                .map(move |(col, value)| (row, col, value))
        })
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.values.iter_mut().enumerate().flat_map(|(row, v)| {
            v.iter_mut()
                .enumerate()
                .map(move |(col, value)| (row, col, value))
        })
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

    pub fn insert_row_at(&mut self, row: usize, values: impl IntoIterator<Item = T>) {
        let new_row: Vec<_> = values.into_iter().take(self.cols()).collect();

        if new_row.len() != self.cols() {
            panic!(
                "Not enough elements. Expected {}, Found {}",
                self.cols(),
                new_row.len()
            )
        }

        self.values.insert(row, new_row);
    }

    pub fn insert_col_at(&mut self, col: usize, values: impl IntoIterator<Item = T>) {
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
            .for_each(|v| v.insert(col, new_col.pop().unwrap()));
    }

    pub fn position<U: TryInto<usize>>(&self, row: U, col: U) -> Option<Position<T>> {
        match (row.try_into(), col.try_into()) {
            (Ok(r), Ok(c)) if r < self.rows() && c < self.cols() => Some(Position {
                row: r,
                col: c,
                grid: self,
            }),
            _ => None,
        }
    }

    pub fn positions(&self) -> impl Iterator<Item = Position<T>> {
        self.enumerate().map(|(row, col, _)| Position {
            row,
            col,
            grid: self,
        })
    }

    pub fn map<U>(&self, map_fn: fn(&T) -> U) -> Grid<U> {
        self.values
            .iter()
            .map(|row_values| row_values.iter().map(map_fn))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn grid() -> Grid<char> {
        let lines = vec!["abc", "def", "ghi", "jkl"];
        lines.iter().map(|l| l.chars()).collect()
    }

    #[rstest]
    fn test_direct_values(grid: Grid<char>) {
        assert!(!grid.values.is_empty());
        assert_eq!(grid.values.len(), 4);
        assert_eq!(grid.values[0].len(), 3);
        assert_eq!(grid.values[0][0], 'a');
        assert_eq!(grid.values[0][1], 'b');
        assert_eq!(grid.values[1][1], 'e');
    }

    #[rstest]
    fn test_get_by_index(grid: Grid<char>) {
        assert_eq!(*grid.get(1, 1), 'e');
    }

    #[rstest]
    fn test_enumerate(grid: Grid<char>) {
        let positions: Vec<_> = grid.enumerate().map(|(a, b, c)| (a, b, *c)).collect();

        assert_eq!(positions[0], (0, 0, 'a'));
        assert_eq!(positions[1], (0, 1, 'b'));
        assert_eq!(positions[4], (1, 1, 'e'));
    }

    #[rstest]
    fn test_rows(grid: Grid<char>) {
        assert_eq!(grid.rows(), 4);
    }

    #[rstest]
    fn test_cols(grid: Grid<char>) {
        assert_eq!(grid.cols(), 3);
    }

    #[rstest]
    #[should_panic]
    fn test_invalid_collect() {
        let lines = vec!["abc", "defg"];
        let _: Grid<_> = lines.iter().map(|l| l.chars()).collect();
    }

    #[rstest]
    fn test_row_wise_iter(grid: Grid<char>) {
        let row: Vec<_> = grid.row_wise_iter().next().unwrap().map(|c| *c).collect();

        assert_eq!(row, vec!['a', 'b', 'c']);
    }

    #[rstest]
    fn test_col_wise_iter(grid: Grid<char>) {
        let col: Vec<_> = grid.col_wise_iter().next().unwrap().map(|c| *c).collect();

        assert_eq!(col, vec!['a', 'd', 'g', 'j']);
    }

    #[rstest]
    fn test_insert_row_at(mut grid: Grid<char>) {
        grid.insert_row_at(1, "123".chars());

        assert_eq!(*grid.get(1, 0), '1');
    }

    #[rstest]
    fn test_insert_col_at(mut grid: Grid<char>) {
        grid.insert_col_at(1, "1234".chars());

        assert_eq!(*grid.get(0, 1), '1');
    }

    #[rstest]
    #[case(-1, 0)]
    #[case(0, -1)]
    #[case(4, 0)]
    #[case(0, 3)]
    fn test_invalid_positions(grid: Grid<char>, #[case] row: i32, #[case] col: i32) {
        assert!(grid.position(row, col).is_none());
    }

    #[rstest]
    #[case(0, 0, 'a')]
    #[case(1, 1, 'e')]
    #[case(3, 0, 'j')]
    fn test_valid_positions(
        grid: Grid<char>,
        #[case] row: i32,
        #[case] col: i32,
        #[case] value: char,
    ) {
        let pos = grid.position(row, col).unwrap();
        assert_eq!(row as usize, pos.row());
        assert_eq!(col as usize, pos.col());
        assert_eq!(value, *pos.value());
    }

    #[rstest]
    #[case(0, 0, Direction::North)]
    #[case(0, 0, Direction::West)]
    #[case(3, 2, Direction::East)]
    #[case(3, 2, Direction::South)]
    fn test_neighbors_invalid_positions(
        grid: Grid<char>,
        #[case] row: usize,
        #[case] col: usize,
        #[case] dir: Direction,
    ) {
        let pos = grid.position(row, col).unwrap();
        assert!(pos.get_neighbor(dir).is_none())
    }

    #[rstest]
    #[case(1, 1, Direction::North, 'b')]
    #[case(1, 1, Direction::East, 'f')]
    #[case(1, 1, Direction::South, 'h')]
    #[case(1, 1, Direction::West, 'd')]
    fn test_neighbors_valid_positions(
        grid: Grid<char>,
        #[case] row: usize,
        #[case] col: usize,
        #[case] dir: Direction,
        #[case] value: char,
    ) {
        let pos = grid.position(row, col).unwrap();
        let neighbor = pos.get_neighbor(dir).unwrap();
        assert_eq!(value, *neighbor.value());
    }

    #[rstest]
    fn test_map(grid: Grid<char>) {
        let mapped = grid.map(|c| c.to_ascii_uppercase());

        assert_eq!('A', *mapped.get(0, 0));
        assert_eq!('B', *mapped.get(0, 1));
        assert_eq!('E', *mapped.get(1, 1));
    }

    #[rstest]
    fn test_new_grid() {
        let g = Grid::new(3usize, 4usize, '.');
        assert_eq!(3, g.rows());
        assert_eq!(4, g.cols());
        assert_eq!('.', *g.get(0, 0));
    }
}
