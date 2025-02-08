use std::collections::HashSet;
pub fn new_count_distinct(input_str: &str) -> usize {
    let strs: Vec<&str> = input_str.split(',').collect();
    let mut ans = 0;
    let mut set = HashSet::<&str>::new();
    for str in strs.into_iter() {
        if !set.contains(&str) {
            set.insert(str);
            ans += 1;
        }
    }

    ans
}
