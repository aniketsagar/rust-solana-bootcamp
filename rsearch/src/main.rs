
/*
 * rsearch- recursive pattern search in a file or list of files in list_dir
 * rserch --p path --s string
 *
 * rsearch --p ./src/main.rs --s "std"
 * output
 *
 *  ./src/main.rs:12:use std::fs;
    ./src/main.rs:13:use std::path::Path;
    ./src/main.rs:22:    let pattern = std::env::args().nth(1).expect("no pattern given");
    ./src/main.rs:23:    let path = std::env::args().nth(2).expect("no path given");


 * 1) find the files in the dir recursively
 * 2) for all the files which we found we search for the string s
 * 3) to do 2 we init a thread pool default 4
 * 4) we maintain a synchronized hashmap  of files and
 * the threads that have picked up that file (f1:{f1:t1},{f2:t2})
 * 5)
 *
 *
 * first version single threaded
 */
use std::fs;
use std::path::Path;

struct result {
    filename: String,//  Path,// or we can use String instead ??
    line: Option<String>,// if there is value Some(str) or synchronized
    line_no: i32
}

impl Drop for result{
    fn drop(&mut self){
        println!("closing file");
    }
}
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    println!("pattern: {:?}, path: {:?}", pattern, path);
    println!("Hello, world!");

    let r = fib(10);
    println!("fib: {r}");
    {
        let a = result{
            filename : "a.txt".to_string(),
            line: None,
            line_no:1
        };
    }
}


fn list_dir(path: &str){

}

fn fib(n:i32)-> i32{
    // 0,1
    // tn = tn-1 + tn-2
    if(n == 0){
        return 0;
    }else if (n == 1){
        return 1;
    }else{
        return fib(n-1)+fib(n-2);
    }
}
