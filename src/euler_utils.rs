pub fn is_prime(candidate: i64) -> bool {
    if candidate == 1 { return false }
    if candidate == 2 { return true }
    if candidate == 5 { return true }
    if candidate % 2 == 0 { return false }
    if candidate % 5 == 0 { return false }
    if candidate <= 10 { return candidate != 9 }
    let mut i: i64 = 11;
    while i*i <= candidate {
        if candidate % i == 0 {
            return false
        }
        i += 2;
    }
    true
}

pub fn sum_vec_i32(iterable: Vec<i32>) -> i32 {
    iterable.iter().sum::<i32>()
}

pub fn factorise_n_by_dividing_divisor_with_n(n: &mut i64, factor: &mut i64, divisor: i64) {
    while *n % divisor == 0 {
        *factor = divisor;
        *n = *n / divisor;
    }
}