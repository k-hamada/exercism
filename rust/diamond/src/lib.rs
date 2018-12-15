pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - 64;
    if n == 1 { return vec![c.to_string()] };

    let mut res = vec![];
    for i in 0..n {
        let mut s = String::new();
        let alphabet = (65 + i) as char;
        let padding = n - i - 1;

        for _ in 0..padding {
            s.push(' ');
        }
        s.push(alphabet);
        if i != 0 {
            let center = 2 * i - 1;
            for _ in 0..center {
                s.push(' ');
            }
            s.push(alphabet);
        }
        for _ in 0..padding {
            s.push(' ');
        }
        res.push(s);
    }

    let mut bottom = res.clone();
    bottom.reverse();
    bottom.remove(0);
    res.append(&mut bottom);

    res
}
