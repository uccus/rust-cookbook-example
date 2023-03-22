use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use rand_distr::{Normal, NormalError};

pub fn test() -> Result<(), NormalError>{
    let mut rng = rand::thread_rng();
    let n1: i32 = rng.gen();
    println!("n1: {}", n1);
    let n2 = rng.gen::<f64>();
    println!("n2: {}", n2);
    
    let n3 = rng.gen_range(0..10);
    println!("n3: {}", n3);
    let n4 = rng.gen_range(0.0..10.0);
    println!("n4: {}", n4);
    println!("n5: {}", rng.gen_bool(1.0 / 3.0));
    
    let d = Uniform::from(1..7);
    loop {
        let n = d.sample(&mut rng);
        println!("n: {}", n);
        if n == 6 {
            break;
        }
    }
    
    let d = Normal::new(2.0, 3.0)?;
    let v = d.sample(&mut rng);
    println!("v: {v}");

    Ok(())
}