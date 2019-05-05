use std::collections::HashMap;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut cells = HashMap::new();
    let mut history = vec![];
    let h = minefield.len();

    for (i, line) in minefield.iter().enumerate() {
        let w = line.len();
        let mut coordinates = vec![];

        for (j, c) in line.chars().enumerate() {
            let point = Point { i, j, h, w };
            coordinates.push(point.coordinate());

            let cell = cells
                .entry(point.coordinate())
                .or_insert_with(Cell::default);
            cell.configure(c);

            if !cell.has_mine {
                continue;
            }
            point.neighbor().iter().for_each(|n_point| {
                cells
                    .entry(n_point.coordinate())
                    .or_insert_with(Cell::default)
                    .increment()
            });
        }
        history.push(coordinates);
    }

    history
        .iter()
        .map(|rows| {
            rows.iter()
                .map(|point| cells.get(&point).unwrap().to_string())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

#[derive(Default)]
struct Cell {
    count: u8,
    has_mine: bool,
}

impl Cell {
    fn configure(&mut self, field: char) {
        self.has_mine = field == '*';
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn to_string(&self) -> String {
        if self.has_mine {
            return String::from("*");
        }
        if self.count == 0 {
            return String::from(" ");
        }
        format!("{}", self.count)
    }
}

struct Point {
    i: usize,
    j: usize,
    w: usize,
    h: usize,
}

impl Point {
    fn coordinate(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    fn neighbor(&self) -> Vec<Point> {
        let mut points = vec![];
        let left = 0 < self.j;
        let right = self.j < self.w - 1;
        let top = 0 < self.i;
        let bottom = self.i < self.h - 1;

        if left && top {
            points.push(Point {
                i: self.i - 1,
                j: self.j - 1,
                w: self.w,
                h: self.h,
            });
        }
        if top {
            points.push(Point {
                i: self.i - 1,
                j: self.j,
                w: self.w,
                h: self.h,
            });
        }
        if top && right {
            points.push(Point {
                i: self.i - 1,
                j: self.j + 1,
                w: self.w,
                h: self.h,
            });
        }
        if left {
            points.push(Point {
                i: self.i,
                j: self.j - 1,
                w: self.w,
                h: self.h,
            });
        }
        if right {
            points.push(Point {
                i: self.i,
                j: self.j + 1,
                w: self.w,
                h: self.h,
            });
        }
        if left && bottom {
            points.push(Point {
                i: self.i + 1,
                j: self.j - 1,
                w: self.w,
                h: self.h,
            });
        }
        if bottom {
            points.push(Point {
                i: self.i + 1,
                j: self.j,
                w: self.w,
                h: self.h,
            });
        }
        if bottom && right {
            points.push(Point {
                i: self.i + 1,
                j: self.j + 1,
                w: self.w,
                h: self.h,
            });
        }

        points
    }
}
