pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    // 2个人选择10个数字，每个人的数字都不一样的概率为: 
    // 10 * 9 / 10^2
    let mut tmp1: f64 = 1_f64;
    for i in 0..n {
        tmp1 *= (365-i) as f64;
    }
    
    let tmp2 = 365_f64.powi(n as i32);

    1_f64 - tmp1 / tmp2
}