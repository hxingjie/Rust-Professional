pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let l = num_str.find('(').unwrap();

    // get from base
    let from_base_t = &num_str[l+1..num_str.len()-1];
    let mut from_base = 0;
    for t in from_base_t.as_bytes().iter() {
        from_base = from_base * 10 + ((*t)-b'0');
    }

    // get num
    let num = &num_str[..l];
    
    // num(from_base) -> num(10)
    let mut num_10: u32 = 0;
    for (i, n) in num.as_bytes().iter().rev().enumerate() {
        num_10 += (*n - b'0') as u32 * from_base.pow(i as u32) as u32;
    }
    
    // num(10) -> num(to_base)
    let mut num: Vec<u8> = Vec::new();
    while num_10 > 0 {
        let tmp = (num_10 % to_base) as u8;
        if tmp < 10 {
            num.push(tmp + b'0');
        } else {
            num.push(tmp-10 + b'a');
        }
        num_10 /= to_base;
    }
    
    // num(to_base) -> String
    let mut ans = String::new();
    for n in num.iter().rev() {
        ans.push(*n as char);
    }

    ans
}