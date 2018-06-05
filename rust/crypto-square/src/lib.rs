pub fn encrypt(input: &str) -> String {
    let chars = normalize(input).chars().collect::<Vec<_>>();
    mapping(square(chars.len()))
        .iter()
        .map(|row| {
            row.iter()
                .map(|&n| chars.get(n).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect()
}

fn square(size: usize) -> (usize, usize) {
    for c in 1.. {
        for r in c - 1..c + 1 {
            if c * r >= size {
                return (c, r);
            }
        }
    }
    return (0, 0);
}

fn mapping((c, r): (usize, usize)) -> Vec<Vec<usize>> {
    (0..c).map(|n| step_by(n, c, r)).collect()
}

fn step_by(offset: usize, step: usize, len: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(len);
    let mut i = 0;
    while res.len() < len {
        res.push(i + offset);
        i += step;
    }
    res
}
