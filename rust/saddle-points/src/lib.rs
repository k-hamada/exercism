pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for (row, n1) in max(&input) {
        for (col, n2) in min(&transpose(&input)) {
            if n1 == n2 {
                result.push((row, col))
            }
        }
    }

    result
}

fn transpose(input: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let mut result = vec![];
    let n = input.len();
    let m = input[0].len();

    for j in 0..m {
        let mut item = vec![];
        for i in 0..n {
            item.push(input[i][j]);
        }
        result.push(item);
    }

    result
}

fn max(input: &[Vec<u64>]) -> Vec<(usize, &u64)> {
    input
        .iter()
        .enumerate()
        .filter(|&(_, t)| t.len() != 0)
        .map(|(i, t)| (i, t.iter().max().unwrap()))
        .collect::<Vec<(_, _)>>()
}

fn min(input: &[Vec<u64>]) -> Vec<(usize, &u64)> {
    input
        .iter()
        .enumerate()
        .filter(|&(_, t)| t.len() != 0)
        .map(|(i, t)| (i, t.iter().min().unwrap()))
        .collect::<Vec<(_, _)>>()
}
