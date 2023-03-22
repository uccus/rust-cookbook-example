use chrono::{DateTime, Duration, Local, Utc};

pub fn test(){
    let now = Utc::now();
    println!("{}", now);
    let now = Local::now();
    println!("{}", now);
    let utc_time = DateTime::<Utc>::from_utc(now.naive_utc(), Utc);
    println!("{}", utc_time);
}