use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator, IntoParallelRefIterator};
use rand::distributions::Alphanumeric;
use rand::Rng;
use rayon::slice::ParallelSliceMut;

struct Person{
    age: i32,
}

pub fn test()
{
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p|{
        *p -= 1
    });
    println!("{:?}", arr);
    
    let arr = [2, 4, 6, 9];
    assert!(arr.par_iter().any(|n| (*n % 2) == 0));
    // assert!(arr.par_iter().all(|n| (*n % 2) == 0));
    
    let res = arr.par_iter().find_any(|&&n| n == 9);
    assert_eq!(res, Some(&9));
    
    // let mut arr = vec![String::new(); 1000];
    // arr.par_iter_mut().for_each(|c|{
    //     let mut rng = rand::thread_rng();
    //     *c = (0..5).map(|_| rng.sample(&Alphanumeric)).map(char::from).collect();
    // });
    // arr.par_sort_unstable();
    // println!("{:#?}", arr);
    
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    s = (0..10).map(|_| rng.sample(&Alphanumeric) as char).collect();
    println!("{}", s);
    
    let v = vec![
        Person{age: 23},
        Person{age: 30},
        Person{age: 31},
        Person{age: 33},
    ];
    
    let v1 = v.par_iter()
                        .map(|x| x.age)
                        .filter(|x| *x > 30)
                        .reduce(|| 0, |x, y| x + y);
    println!("{:#?}", v1);
    
    let v1: i32 = v.par_iter()
                        .map(|x| x.age)
                        .filter(|x| *x > 30)
                        .sum();
    println!("{:#?}", v1);
}