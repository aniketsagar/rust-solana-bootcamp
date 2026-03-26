/*
The Fibonacci sequence begins with [0, 1]. For n > 1, the next number is the sum of the previous two.

Write a function fib(n) that calculates the nth Fibonacci number. When will this function panic?

 The Fibonacci sequence is a sequence where the next term is the sum of the previous two terms. The first two terms of the Fibonacci sequence are 0 followed by 1. The Fibonacci sequence: 0, 1, 1, 2, 3, 5, 8, 13, 21

The Fibonacci sequence is defined as follows:

F(0) = 0
F(1) = 1
F(n) = F(n - 1) + F(n - 2) for n > 1
Examples :

Input: n = 5
Output: 5
Explanation: The 5th Fibonacci number is 5.
Input: n = 0
Output: 0
Explanation: The 0th Fibonacci number is 0.
Input: n = 1
Output: 1
Explanation: The 1st Fibonacci number is 1.
Constraints:
0 ≤ n ≤ 30
 */

use std::convert::TryInto;
fn main() {
    // N {0 30}
    println!("hi");
    let n = 46;
    let result = fib(n);
    println!("result {}", result);

    println!("hi111");
    let n = 10;
    let result = fib_memoized(n);
    println!("result {}", result);

    println!("hi111111111");
    let n = 0;
    let result = fib_iter_table(n);
    println!("result {}", result);
    let n = 1;
    let result = fib_iter_table(n);
    println!("result {}", result);
    let n = 2;
    let result = fib_iter_table(n);
    println!("result {}", result);
    let n = 46;
    let result = fib_iter_table(n);
    println!("result {}", result);
}

fn fib(n: u32) -> u32 {
    // we want to find the nth fibonacci number
    // Tn = Tn-1 + Tn-2
    // T0 = 0
    // T1 = 1
    // 0, 1, 1 ,2, 3, 5, 8

    let mut fib_n = 0;
    if n < 2 {
        return n;
    } else {
        fib_n = fib(n - 1) + fib(n - 2);
    }
    fib_n
}

fn fib_memoized(n: i32) -> i32 {
    // 0 < N  < 30
    let mut table = [-1; 50];
    let mut fib_n = 0;
    table[0] = 0;
    table[1] = 1;

    if n < 2 {
        return n;
    } else {
        let index: usize = n.try_into().unwrap(); // just to cast  n to usize
        //let index: usize = n;
        if table[index] == -1 {
            table[index] = fib_memoized(n - 1) + fib_memoized(n - 2);
            table[index]
        } else {
            table[index]
        }
    }
}

fn fib_iter_table(n: i32) -> i32 {
    // 0<n<30
    let mut table: [i32; 50] = [-1; 50];
    // now we want to build the solution bottoms up
    // so in recursion we start at n and then go down to 1
    // and evaluate the values from there
    // here we want to evaluate
    // f1 f2 f3 f4 .. and store the result and
    // refer to the result when we want to use them

    if n < 2 {
        n
    } else {
        for i in 0..=n {
            let index: usize = i.try_into().unwrap();
            if i < 2 {
                table[index] = i;
            } else {
                table[index] = table[index - 1] + table[index - 2];
            }
        }
        println!("n = {}", n);
        let res = n;
        let index: usize = res.try_into().unwrap();
        table[index]
    }
}
