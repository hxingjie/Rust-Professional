fn is_valid(num: i32, is_prime: &Vec<bool>) -> bool {
    for prime in 2..num { // 先遍历素数
        if is_prime[prime as usize] {
            let tmp = (num - prime) / 2;

            let mut j = 1;
            while j * j <= tmp { // 在遍历平方数
                if j * j == tmp {
                    return false;
                }
                j += 1;
            }
        }
    }
    true
}

pub fn goldbach_conjecture() -> String {
    // 使用素数筛算法先得到素数
    let mut is_prime = vec![true; 6000];
    for i in 2..6000 {
        if is_prime[i] {
            let mut j = i * i;
            while j < 6000 {
                is_prime[j] = false;
                j += i;
            }
        }
    }

    let mut ans = String::new();
    let mut num = 2;
    let mut counter = 0;
    
    loop {
        if num % 2 != 0 && !is_prime[num as usize] && is_valid(num, &is_prime) {
            // `奇数` && `合数` && 不能由 `一个素数` + `一个平方的两倍` 得到的数
            counter += 1;
            ans.push_str(&num.to_string());

            if counter == 1 {
                ans.push_str(",");
            } else if counter == 2 {
                break;
            }
        }
        num += 1;
    }
    
    ans
}
