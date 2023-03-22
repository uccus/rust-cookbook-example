use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn test(){
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("password: {}", password);
}