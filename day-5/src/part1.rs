use regex::Regex;

pub struct Map {
    map: Vec<Vec<i32>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Line {
    points: Vec<Point>,
}

pub fn parse_coordinates(data: &[String]) -> (Vec<Line>, usize) {
    let mut lines: Vec<Line> = Vec::new();
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    let mut max_size = 0;
    for line in data {
        let caps = re.captures(line).unwrap();
        let start = Point {
            x: caps["x1"].parse().unwrap(),
            y: caps["y1"].parse().unwrap(),
        };
        let end = Point {
            x: caps["x2"].parse().unwrap(),
            y: caps["y2"].parse().unwrap(),
        };
        if start.x > max_size {
            max_size = start.x
        };
        if start.y > max_size {
            max_size = start.y
        };
        if end.x > max_size {
            max_size = end.x
        };
        if end.y > max_size {
            max_size = end.y
        };
        if let Some(new_line) = Line::new(start, end) {
            lines.push(new_line);
        }
    }
    (lines, max_size)
}

fn generate_diagonal(start: Point, end: Point) -> Option<Vec<Point>> {
    let mut points: Vec<Point> = Vec::new();
    if start.x < end.x {
        if start.y < end.y {
            let x_range = (start.x..(end.x + 1)).collect::<Vec<usize>>();
            let y_range = (start.y..(end.y + 1)).collect::<Vec<usize>>();
            points.extend(generate_diagonal_points(x_range, y_range));
        } else if start.y > end.y {
            let x_range = (start.x..(end.x + 1)).collect::<Vec<usize>>();
            let y_range = (end.y..(start.y + 1)).collect::<Vec<usize>>();
            points.extend(generate_diagonal_points(x_range, y_range));
        }
    } else if start.x > end.x {
        if start.y < end.y {
            let x_range = (end.x..(start.x + 1)).collect::<Vec<usize>>();
            let y_range = (start.y..(end.y + 1)).collect::<Vec<usize>>();
            points.extend(generate_diagonal_points(x_range, y_range));
        } else if start.y > end.y {
            let x_range = (end.x..(start.x + 1)).collect::<Vec<usize>>();
            let y_range = (end.y..(start.y + 1)).collect::<Vec<usize>>();
            points.extend(generate_diagonal_points(x_range, y_range));
        }
    }
    match points.len() {
        0 => None,
        _ => Some(points),
    }
}

fn generate_diagonal_points(x_range: Vec<usize>, y_range: Vec<usize>) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    if x_range.len() == y_range.len() {
        for (x, y) in x_range.into_iter().zip(y_range.into_iter()) {
            points.push(Point { x, y });
        }
    }
    points
}

fn generate_horizontal(start: Point, end: Point) -> Option<Vec<Point>> {
    let mut points: Vec<Point> = Vec::new();
    if start.x == end.x {
        if start.y > end.y {
            for i in end.y..(start.y + 1) {
                points.push(Point { x: start.x, y: i })
            }
        } else if start.y < end.y {
            for i in start.y..(end.y + 1) {
                points.push(Point { x: start.x, y: i })
            }
        }
    } else if start.y == end.y {
        if start.x > end.x {
            for i in end.x..(start.x + 1) {
                points.push(Point { x: i, y: start.y })
            }
        } else if start.x < end.x {
            for i in start.x..(end.x + 1) {
                points.push(Point { x: i, y: start.y })
            }
        }
    }
    match points.len() {
        0 => None,
        _ => Some(points),
    }
}

impl Line {
    fn new(start: Point, end: Point) -> Option<Line> {
        let mut points: Vec<Point> = Vec::new();
        if let Some(new_points) = generate_horizontal(start, end) {
            points.extend(new_points);
        }
        //if points.len() == 0 {
        //  if let Some(new_points) = generate_diagonal(start, end) {
        //    points.extend(new_points);
        //}
        //}

        Some(Line { points })
    }
}

impl Map {
    fn new(map_size: usize) -> Map {
        let map = vec![vec![0i32; map_size]; map_size];
        Map { map }
    }
    fn get_map(&self) -> &Vec<Vec<i32>> {
        &self.map
    }

    fn display_map(&self) {
        for row in &self.map {
            for val in row {
                print!("{}", val)
            }
            print!("\n")
        }
    }

    fn add_lines(&mut self, lines: &Vec<Line>) {
        for line in lines {
            for point in &line.points {
                self.map[point.y][point.x] += 1;
            }
        }
    }
    fn calculate_overlaps(&self) -> i32 {
        let mut overlaps = 0;
        for row in &self.map {
            for val in row {
                if *val >= 2 {
                    overlaps += 1;
                }
            }
        }
        overlaps
    }
}

pub fn calculate_two_line_overlap(data: &[String]) -> i32 {
    let (lines, max_size) = parse_coordinates(&data);
    let mut map = Map::new(max_size + 1);
    map.add_lines(&lines);
    //map.display_map();
    map.calculate_overlaps()
}
