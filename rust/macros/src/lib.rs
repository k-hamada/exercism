#[macro_export]
macro_rules! hashmap {
    () => {
        HashMap::new()
    };
    ( $($key:expr => $value:expr),* $(,)* ) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}
