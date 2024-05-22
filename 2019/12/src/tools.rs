pub fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd_of_two_numbers(b, a % b)
    }
}

pub fn lcm_of_two_numbers(a: usize, b: usize) -> usize {
    a * b / gcd_of_two_numbers(a, b)
}

pub fn lcm_of_n_numbers(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm_of_n_numbers(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}
