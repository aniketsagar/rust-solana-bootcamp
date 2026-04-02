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

    let opt = first_word_short_version(s);
    dbg!(opt);

    let opt = last_char(s);
    dbg!(opt);

    let opt = last_word_length("rust");
    dbg!(opt);

    let opt111 = find_first_even_number_fold(vec![1, 3, 8, 5]);
    dbg!(opt111);
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

fn first_word_short_version(s: &str) -> Option<String> {
    s.split_whitespace().next().map(|x| x.to_string())
}

fn last_char(s: &str) -> Option<char> {
    // return Some(last charachter)
    // so  last_char("rust") => Some('t')
    // and none otherwise
    // we are using and_then here

    s.split_whitespace().last().and_then(|x| x.chars().last())
}

fn last_word_length(s: &str) -> Option<usize> {
    s.split_whitespace().last().map(|word| word.len())
}

/// already found → keep it
// not found → check current element
fn find_first_even_number_fold(nums: Vec<i32>) -> Option<i32> {
    nums.iter().fold(None, |mut acc, x| match acc {
        Some(_) => acc,
        None => match *x % 2 {
            0 => Some(*x),
            _ => None,
        },
    })
}
