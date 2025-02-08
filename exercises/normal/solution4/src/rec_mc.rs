pub fn dp_rec_mc(amount: u32) -> u32 {
    let mut ans: u32 = 0;
    let mut amount = amount;
    while amount > 0 {
        match amount {
            t if t >= 100 => amount -= 100,
            t if t >= 50 => amount -= 50,
            t if t >= 30 => amount -= 30,
            t if t >= 20 => amount -= 20,
            t if t >= 10 => amount -= 10,
            t if t >= 5 => amount -= 5,
            t if t >= 2 => amount -= 2,
            t if t >= 1 => amount -= 1,
            _ => panic!("error"),
        }
        ans += 1;
    }

    ans
}