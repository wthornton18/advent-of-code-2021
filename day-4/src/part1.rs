

pub fn parse_boards(data: &[String]) -> (Vec<i32>, Vec<Board>) {
    let numbers: Vec<i32> = data[0].split(",").map(|x| x.parse().unwrap()).collect();
    let mut index = 2;
    let mut boards: Vec<Board> = Vec::new();
    while index < data.len() {
        let mut state = vec![vec![0i32;5];5];
        for i in 0..5 {
            let row: Vec<i32> = data[index+i].split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
            for j in 0..5 {
                state[i][j] = row[j]
            }
        }
        index += 6;
        boards.push(Board::new(state))

    }
    return (numbers, boards)
}

#[derive(Clone, Debug)]
pub struct Board {
    state: Vec<Vec<i32>>,
    called_points: Vec<Vec<i32>>,
    n: usize,
    m: usize
}

impl Board {
    fn new(state: Vec<Vec<i32>>) -> Board {
        let n = state.len();
        let m = state[0].len();
        let called_points = vec![vec![0i32;n];m];
        Board {state, called_points, n, m}
    }

    pub fn get_state(&self) -> &Vec<Vec<i32>> {
        &self.state
    }

    pub fn get_checked(&self) -> &Vec<Vec<i32>> {
        &self.called_points
    }

    fn transpose(&self) -> Vec<Vec<i32>> {
        let mut transpose_board = vec![vec![0i32;self.m];self.n];
        let clone_state = &self.state.clone();
        for (i, row) in clone_state.into_iter().enumerate() {
            for (j, val) in row.into_iter().enumerate() {
                transpose_board[j][i] = *val
            }
        }
        transpose_board
    }

    fn unmarked_sum(&self) -> i32 {
        let clone_points = &self.called_points.clone();
        let mut sum = 0;
        for (i, row) in clone_points.into_iter().enumerate() {
            for (j, val) in row.into_iter().enumerate() {
                if *val != 1 {
                    sum += self.state[i][j]
                }
            }
        }
        sum
    }

    fn is_won(&self) -> bool {
        for row in &self.called_points {
            if row.iter().sum::<i32>() == self.n as i32 {
                return true;
            }
        }
        for column in self.transpose() {
            if column.iter().sum::<i32>() == self.m as i32 {
                return true;
            }
        }
        false
    }
    
    fn update_called_points(&mut self, number: i32) {
        let clone_state = self.state.clone();
        for (i, row) in clone_state.into_iter().enumerate() {
            for (j, val) in row.into_iter().enumerate() {
                if val == number {
                    self.called_points[i][j] = 1;
                }
            }
        }   
    }

}

fn print_board(board: &mut Board) {
    let state = board.get_state();
    let checked_squares = board.get_checked();
    print!("\n");
    for (i, row) in state.into_iter().enumerate() {
        for (j, val) in row.into_iter().enumerate() {
            if checked_squares[i][j] == 1 {
                print!("-1 ")
            } else {
                print!("{} ", val);
            }
            
        }
        print!("\n");
    }

}

pub fn calculate_winner(input: &[String]) -> Result<i32, &'static str> {
    let (numbers, mut boards) = parse_boards(&input);
    for number in numbers {
        for b in &mut boards {
            b.update_called_points(number);
            if b.is_won() {
                let b_sum = b.unmarked_sum();
                return Ok(number * b_sum)
            }
        }
    }
    Ok(0)
}