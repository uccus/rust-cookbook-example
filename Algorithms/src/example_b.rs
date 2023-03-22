use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
pub struct Point{
    pub x: i32,
    pub y: i32,
}

impl Distribution<Point> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point{
        let (x, y) = rng.gen();
        Point { x, y }
    }
}

pub fn test(){
    let mut rng = rand::thread_rng();
    let p = rng.gen::<Point>();
    println!("p: {:#?}", p);
}