/*
* An iterator is a machenism by which we generate values lazily.
* Ex let A = Vec![1,3,4,2,2,5,5,2]
* now if we want to iterate i.e. read the values of A one at a time in sequence
* then we can either store all the values in memory at the same time or
* we can move in the same way a linked list works that we have only sufficiant data
* to visit the next node at any given time.
*
* iterators in rust works in the second manner. so A.iter() doesn't give data
* directly it is method associated with that instance of data for the given iterable structure
*
* so A.iter() gives us a machine that spits out the data via next() function ( next is mendatory for an iterator
* to be valid)
* or any other functions such as last().
*
* in most cases once we use A.iter().next() the value is popped and cannot be visited again via same
* iterator.
*
* This type of just in time evalution is called lazy evaluation. the opposite is called eger evaluation.
*
* trait Iterator {
       type Item;

       fn next(&mut self) -> Option<Self::Item>;
   }


    Method	        Returns	    Consumes vector
    iter()	        &T	        ❌
    iter_mut()	    &mut T	    ❌
    into_iter()	    T	        ✅

    Iterator Consumers  ::

    let iter = v.iter();
    now iter is an iterator bound to v
    but this line doesnt do anything it executes only when a consumer function like next(), last() or collect()
    is called on that iterator;

    iterators are executed lazily so until a consumer is called to consume a value nothing happens

    Consumers that collapse iterator into single value:
    | Consumer    | Output    |
    | ----------- | --------- |
    | `sum()`     | number    |
    | `count()`   | usize     |
    | `fold()`    | custom    |
    | `product()` | number    |
    | `max()`     | Option<T> |
    | `min()`     | Option<T> |

*/

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = vec![1, 3, 4];

    // iterator type 1 - read only
    let mut values = v.iter(); // this is mutable because iter().next() require &mut self
    // next(&mut self)
    // Rust iterators store internal state, and calling next() changes that state.
    // that is the iterator itself is changing and hence it must be mutable
    // when we  call iter().next() then iter() updates the index from which it is reading
    // and so it needs to be mutable
    // the data however is not changing here. as it is returning &1 &2 &3 etc
    //
    //
    // Example mental model:
    // vector = book
    //  iterator = bookmark
    //  Reading moves the bookmark.
    //  So bookmark must be mutable.
    //  Book does NOT change.
    println!("{:?}", values.next());
    dbg!(v.iter().rev().next());
    /*
    * why does in the below for loop there is no
    * error about mutable iterator
    * why don't we apply &mut anywhere in v.iter()
    *
    * Answer : because rust is internally doing this

    *   let mut iter = v.iter();

    *       while let Some(x) = iter.next() {
    *           println!("{}", x);
    *      }
    *
    * compiler inserted mut iter internally
    */
    for x in v.iter() {
        dbg!(x);
    }
    //let  = v.iter().find(| **x > 15);

    // Mutable borrow iteration
    // iter_mut gives Iterator <Item = &mut i32>
    // Self::Item is < &mut T>
    let mut iter_mut = v.iter_mut(); // Iterator <Item = &mut i32> 
    let m = iter_mut.next();

    //Ownership iteration
    // into_iter takes the ownership of the values and
    // consumes the structure its iterating on

    let v1 = vec![2, 3, 4, 5, 6];
    for x in v1.into_iter() {
        dbg!(x);
    }
    // now we cannot use v1 as it is consumed
    //v.iter().copied().map().filter().find().collect::<Vec>;

    let s: i32 = v.iter().map(|x| *x * 7).filter(|x| *x > 10).sum();
    dbg!(s);

    // here notice how sum is called
    // we are infering type in the call itself and not for the variable
    // this is closer to sum signeture
    // sum<S>(self) -> S
    let s2 = v.iter().map(|x| *x * 7).filter(|x| *x > 10).sum::<i32>();
    dbg!(s2);

    // Iterator consumer
    // fold -> custom type
}
