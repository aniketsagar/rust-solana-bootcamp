fn main() {
    println!("Hello, world!");
    let even = find_even(21);
    match even {
        Some(x) => println!("Even number found : {}", x),
        None => println!("Odd number found:"),
    }

    match find_even(2) {
        Some(x) => println!("Even number found: {}", x),
        None => println!("Odd numeber found:"),
    }

    let v = vec![1, 3, 9, 6, 4];
    match find_first_even_number(v) {
        Some(n) => println!("Found first even number: {}", n),
        None => println!("No even number found in vector"),
    }
    let s = "hello world";
    dbg!(first_word(s));

    // option mapping
    // option.map and option.and_then
    // both work on Some(T)
    //.map() = apply function IF value exists
    let opt = Some(2).map(|x| x * 10);
    dbg!(opt);
}

fn find_even(n: i32) -> Option<i32> {
    // here we spacify concrete option
    if n % 2 == 0 { Some(n) } else { None }
}

fn find_first_even_number(nums: Vec<i32>) -> Option<i32> {
    for num in nums {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn first_word(s: &str) -> Option<String> {
    match s.split_whitespace().next() {
        Some(x) => Some(x.to_string()),
        None => None,
    }
}
