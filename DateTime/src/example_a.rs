use std::time::{Duration, Instant};
use std::thread;

fn expensive_sample(){
    thread::sleep(Duration::from_millis(3000));
}

pub fn test(){
    let start = Instant::now();
    expensive_sample();
    let end = start.elapsed();
    println!("{:?}", end);
}