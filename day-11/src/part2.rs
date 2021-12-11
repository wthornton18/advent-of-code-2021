use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Floor {
    grid: Vec<i32>,
    width: usize,
    height: usize,
}


impl Floor {
    pub fn new(lines: &[String]) -> Floor {
        let width = lines[0].len();
        let height = lines.len();
        let mut grid: Vec<i32> = Vec::new();
        for line in lines {
            for c in line.chars() {
                grid.push(c as i32 - '0' as i32)
            }
        }
        Floor { grid, width, height}
    }

    pub fn board_size(&self) -> usize {
        self.width * self.height
    }

    pub fn index(&self, x: usize, y: usize) -> Option<usize> { 
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            return Some((y * self.width )+ x)
        }
        None
    }
    pub fn get_val(&self, x: usize, y: usize) -> Option<i32> {
        if let Some(point) = self.index(x, y) {
            return Some(self.grid[point])
        }
        None
    }

    pub fn get_surrounding_indexes(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut points = HashSet::new();
        for dx in vec![-1, 0, 1] {
            for dy in vec![-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue
                }
                let xi = x as i32 + dx;
                let yi = y as i32 + dy;
                if let Some(_) = self.index(xi as usize, yi as usize) {
                    points.insert((xi as usize, yi as usize));
                }
            }
        }
        points
    }
    pub fn get_all_indexes(&self) -> HashSet<(usize, usize)> {
        let mut points = HashSet::new();
        for y in 0..self.height {
            for x in 0..self.width {
                points.insert((x, y));
            }
        }
        points
    }

    pub fn print_board(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_val(x, y).unwrap());
            }
            print!("\n");
        }
    }

    pub fn check_octupus(&mut self) -> HashSet<(usize, usize)> {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        let mut to_check: HashSet<(usize, usize)> = self.get_all_indexes();
        loop {
            let mut next_check: HashSet<(usize, usize)> = HashSet::new();
            for (x, y) in to_check.clone() {
                if let Some(val) = self.get_val(x, y) {
                    if val > 9 && ! flashed.contains(&(x, y)){
                        flashed.insert((x, y));
                        let surrounding_indexes = self.get_surrounding_indexes(x, y);
                        self.increase_energy(&surrounding_indexes);
                        for (xi, yi) in surrounding_indexes {
                            next_check.insert((xi, yi));
                        }
                    }
                }
            }
            if next_check.is_empty() {
                break
            } else {
                to_check = next_check;
            }
        }
        flashed
    }

    pub fn set_to_zero(&mut self, indexes: &HashSet<(usize, usize)>) {
        for (x, y) in indexes {
            if let Some(index) = self.index(*x, *y) {
                self.grid[index] = 0;
            }
        }
    }


    pub fn increase_energy(&mut self, indexes: &HashSet<(usize, usize)>) {
        for (x, y) in indexes {
            if let Some(index) = self.index(*x, *y) {
                self.grid[index] += 1;
            }
        }
    }

    pub fn step(&mut self) -> usize {
        self.increase_energy(&self.get_all_indexes());
        let flashed = self.check_octupus();
        self.set_to_zero(&flashed);
        flashed.len()
    }

    pub fn calculate_first_synchro_flash(&mut self) -> i32 {
        let size = self.board_size();
        let mut step = 0;
        loop {
            let flashed = self.step();
            if flashed == size {
                return step+1;
            }
            step += 1;
        }
    }
}
