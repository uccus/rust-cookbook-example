use chrono::{Datelike, Timelike, Utc};

pub fn test(){
    let now = Utc::now();
    let (is_pm, hour) = now.hour12();    
    println!("{}", now.to_rfc2822());
    println!("{}", now.to_rfc3339());
    println!("{}", now.format("%Y%m%d-%H:%M:%S.%3f"));
}