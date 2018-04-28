pub struct PascalsTriangle {
    n: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { n: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = vec![];
        for i in 1..self.n+1 {
            let mut row = vec![];
            for j in 0..i {
                row.push(self.c(i-1, j));
            }
            rows.push(row);
        }
        rows
    }

    fn c(&self, n: u32, m:u32) -> u32 {
        self.f(n) / (self.f(m) * self.f(n - m))
    }

    fn f(&self, n: u32) -> u32 {
        if n == 0 {
            1
        } else {
            n * self.f(n - 1)
        }
    }
}
