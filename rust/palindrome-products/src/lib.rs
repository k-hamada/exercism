use std::collections::HashSet;

pub type Palindrome = u64;

pub fn get_palindrome_products(min: u64, max: u64) -> Vec<Palindrome> {
    let range = (min..max+1).collect::<Vec<_>>();
    let products = range.iter().flat_map(|n|
        range.iter().filter_map(move |m| {
            let product = n * m;
            if is_palindromic(product) { Some(product) } else { None }
        })
    ).collect::<HashSet<_>>();
    products.into_iter().collect::<Vec<_>>()
}

pub fn min(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().min().map(|n| *n)
}

pub fn max(palindromes: &[Palindrome]) -> Option<Palindrome> {
    palindromes.iter().max().map(|n| *n)
}

fn is_palindromic(n: u64) -> bool {
    let mut base = n;
    let mut result = 0;
    while base > 0 {
        result = result * 10 + base % 10;
        base /= 10;
    }
    n == result
}
