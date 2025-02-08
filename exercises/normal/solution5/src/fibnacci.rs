pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    if threshold <= 1 {
        return 0;
    }

    let mut ans = 1;

    let mut num_2 = 0;
    let mut num_1 = 1;

    loop {
        let num = num_2 + num_1;
        if num < threshold && num % 2 != 0 {
            ans += num;
        } else if num >= threshold {
            return ans
        }
        num_2 = num_1;
        num_1 = num;
    }
}