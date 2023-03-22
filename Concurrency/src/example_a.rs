use std::thread;
use crossbeam::scope;

pub fn test(){
    let arr = vec![1,2,3,100,3];
    let max = findMax(&arr);
    assert_eq!(max, Some(100));
}

fn findMax(arr: &[i32]) -> Option<i32>{
    println!("当前线程：{:?}", thread::current().id());
    const THRESHOLD: usize = 2;
    if arr.len() < THRESHOLD {
        return Some(arr[0]);
    }
    
    let (left, right) = arr.split_at(arr.len() / 2);
    scope(|s|{
        let thread_l = s.spawn(|_| findMax(left));
        let thread_r = s.spawn(|_| findMax(right));
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;
        Some(max_l.min(max_r))
    }).unwrap()
}
