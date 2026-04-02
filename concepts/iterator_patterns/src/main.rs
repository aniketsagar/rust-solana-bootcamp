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

    v.iter().map().filter().fold(0, |acc, x| acc + x)
}

//  given let v = vec![10,20,30]
//   v.iter() -> Iterator < Item = &i32>
//   Self::Item = &i32
//   .map(). Recieves Self::Item  = &i32
//   |x| *x produces  Self::Item = i32
//   Self::Item = i32
//   filter(). Recives Self::Item = i32
//   returns Self::Item = i32
//   fold(0,|acc, x| acc+ x)

//  now for  fold(0,|acc, x| acc+ x)
//  below is the signature of fold from refernce

// pub fn fold<B, F>(self, init: B, mut f: F) -> B
// where
//     Self: Sized,
//     F: FnMut(B, Self::Item) -> B,

// so fold recievs Self::Item = i32

// so in the closure |acc, x| acc+x

// acc is intialized to 0

// acc -> i32 ( rust is infering type here ? )
// x recieves Self::Item = i32

// so acc -> i32 and x -> i32

// from the description of fold
// it initialize the accumalator with the intial value and
// value of second argument is taken from Self::Item
// and then closure body is executed, the result is stored in accumalator
// and  persisted while the whole structure is iterated over

// so  assuming v as given

// the sequence is as follows
// v -> [10,20,30]
// v.iter -> [&10,&20,&30]
// .map(|x|*x) -> [10,20,30]
// .filter(|x| *x > 10) -> [20,30]
// .fold(0,|acc, x| acc + x)
// iteration   acc x   result
// 0           0
// 1           0   20    20
// 2           20  30    50

// final result 50

// v.iter() -> Iterator<Item=&i32>

//  .filter(|x| **x > 10) -> filter(Predecate(&Self::Item) -> bool)
//  Self::Item = &i32
//  |x| recieves &&i32
//  filter doesn't change Self::Item
//  Self::Item -> &i32

//  .fold(Vec::new(), |mut acc, x| {
//      acc.push(*x);
//      acc
//  })
// pub fn fold<B, F>(self, init: B, f: F) -> B
// where
//     F: FnMut(B, Self::Item) -> B

// so from signeture

// closure recieves (B, Self::Item)

// type of x Self::Item = &i32
// type of acc  -> Vec<i32>

// infered from
// acc.push(*x)  since x is &i32 *x is i32

// final return type
// is Vec<i32>

// v.iter()
//  .map(|x| x + 1)
//  .fold(String::new(), |mut acc, x| {
//      acc.push_str(&x.to_string());
//      acc
//  })

//  v.iter() -> Iterator<Item= i32>
//  so Self::Item = &i32
//  .map(|x| x + 1)
//  closure recieves Self::Item -> &i32
//  map transforms Self::Item -> i32  ( infered from |x| x+1
//     as Add<i32> is implemented for i32
//     rust internally dereferences &x ->  *x and add x + 1
//     so *&i32 -> i32
// )
// Self::Item -> i32
//  .fold(String::new(), |mut acc, x| {
//      acc.push_str(&x.to_string());
//      acc
//  })

//  closure in fold recieves  Self::Item for x

//  type(x) -> Self::Item = i32

//  type(acc) String from initial value

//  final return type
// fold<B, F>(self, init: B, f: F) -> B

// |mut acc, x| {
//      acc.push_str(&x.to_string());
//      acc
//  }

//  from closure we know that &x is converted to string
//  and pushed to acc

//  so final return type is String

// v.into_iter()
//  .map(|x| x + 1)
//  .fold(Vec::new(), |mut acc, x| {
//      acc.push(x);
//      acc
//  })

// v.into_iter() -> Iterator<Item = i32> as it owns the values of the vector
// Self::Item = i32
//  .map(|x| x + 1)
//  FnMut( Self::Item )-> B
//  so |x| x+1 recieves i32 and produces i32

//  so Self::Item = i32

//  .fold(Vec::new(), |mut acc, x| {
//      acc.push(x);
//      acc
//  })

//  now

//  type(x) -> Self::Item -> i32
//  type (acc) -> Vec<_> intialy

//  now from the closure body
//     (Vec::new(), |mut acc, x| {
//         acc.push(x);
//         acc
//     })

// x is pushed into acc which is Vec<_>

// Vec<_> is inferred to be Vec<Self::Item>
// Vec<i32>

// type(acc) Vec<i32>

// finally acc is returned

// final return type is Vec<i32>

//  fn into_iter(self) -> Self::IntoIter

//  v.iter()
//  .map(|x| x + 1)
//  .fold(Vec::new(), |mut acc, x| {
//      acc.push(x);
//      acc
//  })

//  v.iter() -> Iterator<Item=&i32>
//  Self::Item = &i32
//  .map(|x| x + 1) recieves Self::Item -> &i32
//  |x| x+1 -> &i32 -> i32 since in standard lib &i32 impl Add<i32> so &i32 +i32 -> i32

//  Self::Item = i32

//  .fold(Vec::new(), |mut acc, x| {
//      acc.push(x);
//      acc
//  })

//  type(x) -> Self::Item -> i32
//  type(acc) -> Vec<_> on init

//  from closure body
//  acc.push(x)

//  rust infers Vec<_> -> Vec<i32>

//  type(acc) -> Vec<i32>
// acc is returned
//  final return type Vec<i32>
