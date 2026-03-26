pub mod basic_node;
pub mod fibonacci;
#[derive(Debug)]
struct counter {
    data: i32,
}

impl counter {
    fn increment(&mut self) {
        self.data = self.data + 1;
    }

    fn get(&self) -> &Self {
        &self
    }
}
fn main() {
    println!("Hello, world!");
    let s1 = String::from("hey there!!");
    read_str(&s1);
    let mut s2 = String::from("Check this string.");
    read_str(&s2); // immutable reference
    // reference last used here
    // mutable reference is valid after last use of immutable
    // reference
    mod_str(&mut s2);
    println!("{:?}", s2);

    let slice1 = first_word(&s2);
    println!("{:?}", slice1);

    let mut unit_counter = counter { data: 1 };
    println!("{:?}", &unit_counter.get());
    unit_counter.increment();
    println!("{:?}", &unit_counter);

    println!("Longer string is {:?}", longest(&s1, &s2));
    println!("Shorter string is {:?}", shortest(&s1, &s2));
    println!("Shorter string is {:?}", shortest(&s2, &s2));

    let result;
    {
        let s3 = String::from("abceeeeeee asdlahdadasdsajdasdadasda");
        result = shortest(&s3, &s1);
        println!("{}", result);
    }
}

/*
Borrowing: We want to use a value temporarily, without owning it.

We do it by getting a reference to it

&T
There are two kinds of references
Immutable &T : This kind cannot change the  value it refers to
They are generaly used to pass on the value to be read

Mutable &mut T : They can modify the value they are refering to
and only one mutable reference can be present at a time for a value


At any point of time we can have

Many immutable references to a value
OR
one mutable reference to a value

But not both


Owner = Wikipedia
&T = Reader
&mut T = Editor


Borrowed values are dropped after they are last used in a program

Slices are reference to part of contagous data

let slice = &s[0..2] means that we are referring to data
of s from index 0 to 1  2 is not included.

let slice = &s[0..=2] refers to data from 0 to 2  ( = sign)

let slice = &s[..2] refers to data from 0 to 1

let slice = &s[3..] refers to data from 3 to n-1 ( end of data)

let slice = &s[..] refers to whole data
*/

fn read_str(s: &String) {
    println!("{:?}", s); // no ownership change here
}

fn mod_str(s: &mut String) {
    s.push_str(" modified");
}

fn first_word(s: &String) -> &str {
    for (i, item) in s.chars().enumerate() {
        if item == ' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/*
Lifetimes: in rust lifetimes are relationships of how objects live with respect to each other
It prevenets the problem of dangaling pointers, a dangling pointer is an issue where we are
refereing to a data which is not present or was dropped.
Rust guarentees that every reference is a valid reference.

Lifetimes are a compile time relationship;
It is a constraint describing how long a reference must be valid with respect to other references
or objects;


 */

// We read below line as follows :
// the function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn shortest<'b>(x: &'b str, y: &'b str) -> &'b str {
    if x.len() < y.len() { x } else { y }
}
