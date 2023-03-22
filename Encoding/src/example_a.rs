use url::form_urlencoded::{byte_serialize, parse};

pub fn test(){
    let urlencoded:String = byte_serialize("I want ❥(^_-)".as_bytes()).collect();
    println!("{}", urlencoded);
    
    let decode: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    println!("{}", decode);
}