fn to_value(s: &str) -> i32 {
    let nums: Vec<_> = s.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    return nums[0] * 10 + nums[nums.len() - 1];
}

fn main() {
    let res: i32 = include_str!("../input.txt")
        .split("\n")
        .map(|s| to_value(s))
        .sum();

    println!("{res}");
}
