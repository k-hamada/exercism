pub fn map_function(list: Vec<i32>, f: &Fn(i32) -> i32) -> Vec<i32>
{
    let mut result = Vec::new();
    for item in list {
        result.push(f(item));
    }
    result
}

pub fn map_closure<F>(list: Vec<i32>, f: F) -> Vec<i32>
    where F: Fn(i32) -> i32
{
    let mut result = Vec::new();
    for item in list {
        result.push(f(item));
    }
    result
}
