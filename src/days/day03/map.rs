#[allow(dead_code)]
pub const OPEN_CELL: char = '.';
#[allow(dead_code)]
pub const TREE_CELL: char = '#';

pub struct Map {
    map: Vec<Vec<char>>,
    x: usize,
    y: usize,
}

#[allow(dead_code)]
impl Map {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        Self {
            map,
            x: 0,
            y: 0,
        }
    }

    pub fn from_str(input: &'static str) -> Self {
        let rows = input.split("\n").map(
            |row| row.chars().collect()
        ).collect();
        Self::new(rows)
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
        self.x %= self.map[self.y].len();
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    pub fn set_xy(&mut self, x: usize, y: usize) {
        self.set_x(x);
        self.set_y(y);
    }

    pub fn reset(&mut self) {
        self.set_xy(0, 0);
    }

    pub fn move_x(&mut self, x: usize) {
        self.x += x;
        self.x %= self.map[self.y].len();
    }

    pub fn move_y(&mut self, y: usize) {
        self.y += y;
    }

    pub fn move_xy(&mut self, x: usize, y: usize) {
        self.move_x(x);
        self.move_y(y);
    }

    pub fn on_tree(&self) -> bool {
        self.map[self.y][self.x] == TREE_CELL
    }

    pub fn at_bottom(&self) -> bool {
        (self.y + 1) == self.map.len()
    }

    fn _print(&self, with_position: bool) {
        for (yk, row) in self.map.iter().enumerate() {
            for (xk, cell) in row.iter().enumerate() {
                if with_position && xk == self.x && yk == self.y {
                    print!("O");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }

    pub fn print(&self) {
        self._print(false)
    }

    pub fn print_with_position(&self) {
        self._print(true)
    }
}