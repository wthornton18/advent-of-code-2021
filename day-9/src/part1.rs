

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
    pub fn get_grid(&self) -> &Vec<i32> {&self.grid}
    pub fn get_width(&self) -> usize {self.width}
    pub fn get_height(&self) -> usize {self.height}

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

    pub fn is_lowest(&self, x: usize, y: usize) -> bool {
        let surrounding_vals = self.get_surrounding_vals(x, y);
        let mut val: Option<i32> = None;
        match self.get_val(x, y) {
            Some(some_val) => {val = Some(some_val)},
            None => return false
        }
        match val {
            Some(val) => {
                for surrounding_val in surrounding_vals {
                    if surrounding_val <= val {
                        return false;
                    }
                }
            },
            None => return false
        }
        
        true
    }

    pub fn get_surrounding_vals(&self, x: usize, y: usize) -> Vec<i32> {
        let mut vals: Vec<i32> = Vec::new();
        let surrounding_indexes = self.get_surrounding_indexes(x, y);
        for (xi, yi) in surrounding_indexes {
            if let Some(val) = self.get_val(xi, yi) {
                vals.push(val);
            }
        }
        vals
    }

    pub fn get_surrounding_indexes(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut points = Vec::new();
        for (dx, dy ) in vec![(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let xi = x as i32 + dx;
            let yi = y as i32 + dy;
            if let Some(_) = self.index(xi as usize, yi as usize) {
                points.push((xi as usize, yi as usize))
            }
        }
        points
    }
    pub fn get_all_indexes(&self) -> Vec<(usize, usize)> {
        let mut points: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                points.push((x, y))
            }
        }
        points
    }

    pub fn print_array(&self) {
        println!("{}", self.width);
        println!("{}", self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get_val(x, y).unwrap())
            }
            print!("\n")
        }

    }

    pub fn get_all_values(&self) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();
        for (x, y) in self.get_all_indexes() {
            if let Some(val) = self.get_val(x, y) {
                values.push(val);
            }
        }
        values

    }

    pub fn get_lowest_values(&self) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();
        for (x, y) in self.get_all_indexes() {
            if self.is_lowest(x, y) {
                if let Some(val) = self.get_val(x, y) {
                    values.push(val)
                }
            }
        }
        values
    }

    pub fn calculate_risk_level(&self) -> i32 {
        let mut risk_level: i32 = 0;
        for val in self.get_lowest_values() {
            risk_level += val + 1;
        }
        risk_level
    }
}
