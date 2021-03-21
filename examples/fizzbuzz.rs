//! An implementation of a fizzbuzz algorithm.
//! It's funny how many attempts it took me to 
//! find out we can just turn condition into 
//! 1 with the "as" converter.
//! 
//! main makes a comparison between 3 implementations of fizzbuzz:
//! 
//!  - fizzbuzz creates a string by appending strings to the exit
//! 
//!  - fizzbuzz_simple use a match expression for each string
//! 
//!  - fizzbuzz_comparison uses if statements
//! 
//! 
//! fizzbuzz_simple is twice as fast than fizzbuzz.
//! 
//! fizzbuzz_comparison seems to be around 2% to 4% slower than the fizzbuzz_simple
//! 

use bytes::Bytes;
use std::str;

use std::time::{Duration, Instant};

fn fizzbuzz(value: usize, fizz: &Bytes, buzz: &Bytes) -> String{
    let value_string = Bytes::from(value.to_string());
    let (a, b) = ((value % 3 == 0) as usize, (value % 5 == 0) as usize);
    let (fizz_len, buzz_len) = (a * fizz.len(), b * buzz.len());
    let value_len = (1-a) * (1-b) * (value_string.len());
    format!("{}{}{}",
        str::from_utf8(value_string.get(0..value_len).unwrap()).unwrap(),
        str::from_utf8(fizz.get(0..fizz_len).unwrap()).unwrap(),
        str::from_utf8(buzz.get(0..buzz_len).unwrap()).unwrap(),
    )
}

fn fizzbuzz_simple(value: usize) -> String{
    match(value % 3, value % 5){
        (0, 0) => String::from("fizzbuzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        (_, _) => value.to_string(),
    }
}

fn fizzbuzz_comparison(value: usize) -> String{
    if value % 15 == 0{
        return String::from("fizzbuzz")
    }
    if value % 3 == 0{
        return String::from("fizz")
    }
    if value % 5 == 0{
        return String::from("buzz");
    }
    value.to_string()
}

fn run_fizzbuzz(iterations: usize) -> (u128, usize){
    let fizz = Bytes::from("fizz");
    let buzz = Bytes::from("buzz");

    // println!("fizzbuzz: Working for {} values", iterations);
    let mut sum = 0;
    let now = Instant::now();
    for i in 0..iterations{
        sum+=fizzbuzz(i, &fizz, &buzz).len();
    }
    (now.elapsed().as_micros(), sum)
}

fn run_fizzbuzz_simple(iterations: usize) -> (u128, usize){
    // println!("fizzbuzz_simple: Working for {}", iterations);
    let mut sum = 0;
    let now = Instant::now();
    for i in 0..iterations{
        sum+=fizzbuzz_simple(i).len();
    }
    (now.elapsed().as_micros(), sum)
}

fn run_fizzbuzz_comparison(iterations: usize) -> (u128, usize){
    // println!("fizzbuzz_comparison: Working for {}", iterations);
    let mut sum = 0;
    let now = Instant::now();
    for i in 0..iterations{
        sum += fizzbuzz_comparison(i).len();
    }
    (now.elapsed().as_micros(), sum)
}


fn main(){
    // Runs 10000 times to get a notion of how many iterations are required for one second
    let iterations = 10000;
    let (elapsed_time, _) = run_fizzbuzz(iterations);
    println!("Checking speed-- {} values. time:{} microseconds", iterations, elapsed_time);

    // iterations is assigned a value that should run for 4 seconds
    let iterations = (4*1000*1000*iterations as u128/elapsed_time) as usize;

    // test fizzbuzz for 4 seconds
    let (elapsed_time, sum) = run_fizzbuzz(iterations);
    println!("fizzbuzz ---Done. Time: {} seconds", (elapsed_time as f64)/(1000.0*1000.0));

    // test fizzbuzz_simple for 4 seconds
    let (elapsed_time2, sum2) = run_fizzbuzz_simple(iterations);
    println!("fizzbuzz_simple ---Done. Time: {} seconds", (elapsed_time2 as f64)/(1000.0*1000.0));

    // proportion of time between
    println!("Proportion fizzbuzz/fizzbuzz_simple is {}", elapsed_time as f64/ elapsed_time2 as f64);

    let (elapsed_time3, sum3) = run_fizzbuzz_comparison(iterations);
    println!("fizzbuzz_comparison ---Done. Time: {} seconds", (elapsed_time3 as f64)/(1000.0*1000.0));

    println!("Proportion fizzbuzz_simple/fizzbuzz_comparison is {}", elapsed_time2 as f64/ elapsed_time3 as f64);

    assert!((sum == sum2) && (sum2 == sum3));
}