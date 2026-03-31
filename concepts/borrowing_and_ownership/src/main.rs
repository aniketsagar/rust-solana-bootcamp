// owned values vs borrowed values
// in rust values move around
// imagine a ball being passed around
// everytime a ball is passed around the owner of the ball changes
// and the privious owner cannot do anything with the ball
// same thing happens with values in rust
//
// In case of borrowing, the value is borrowed by the new owner temperelily
// and is returned to the original owner after the use
// like renting a car for travel,
//

// 🧠 Mental Model: Strings Are Books, &str Is a Bookmark

// when we are asking the questions leading with "what"
// ex. what is the last word in the string ? or what is the first name ?
// here we are only reading the data, so we use slices &str over str or String
// but when we want to update or change the data then we might need to take
// ownership or &mut reference and do it

//
// we usually use or return String and not &str when we want to return something
// that doesn't exist in memory already
// so we give or take ownership of something only when
// we want to create something which doesn't exist already in the memory
//

//allocate
// copy
// transform
// return
// If transformation changes content → returns String
// If transformation changes view only → returns &str

fn main() {
    println!("Hello, world!");
    let s = "first string";
    dbg!(first_char(s));
    dbg!(last_char(s));
    dbg!(fast_last_non_space_char(s));
    dbg!(first_word_slice(s));
}

// we are trying to take a string slice instead of string
// because if we use &str the function accepts
// &str , str , String

fn first_char(s: &str) -> Option<char> {
    s.trim().chars().next()
}

fn last_char(s: &str) -> Option<char> {
    // O(n) time complexity because chars().last() needs to traverse
    // the whole array

    // O(1) time complexity chars().rev().next()
    // since rev() is just a starting pointer reversal as str size and start pointer are known
    // so this is faster
    s.trim().chars().last()
}

fn fast_last_non_space_char(s: &str) -> Option<char> {
    s.trim().chars().rev().next()
}

// following is equivelent to
// fn first_word_slice<'a>(s: &'a str) -> Option<&'a str>
fn first_word_slice(s: &str) -> Option<&str> {
    s.trim().split_whitespace().next()
}
