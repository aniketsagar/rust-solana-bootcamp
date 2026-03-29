fn conditional_multiply(nums: &mut Vec<i32>) {
    // Multiply even numbers by 3, odd numbers by 2
    nums.iter_mut().for_each(|num| match *num % 2 {
        0 => {
            *num = *num * 3;
        }
        1 => {
            *num = *num * 2;
        }
        _ => {}
    });
}

pub fn first_word(s: &str) -> String {
    let result = s.split_whitespace().next();
    match result {
        Some(x) => x.to_string(),
        None => s.to_string(),
    }
}
pub fn last_word(s: &str) -> String {
    // return the last word of a whitespace sperated string
    // return whole string if no whitespace
    // hello world -> world
    // helloworld -> helloworld

    let result = s.split_whitespace().last();
    match result {
        Some(x) => x.to_string(),
        None => s.to_string(),
    }
}
