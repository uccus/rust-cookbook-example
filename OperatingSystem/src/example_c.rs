use error_chain::error_chain;
use url::*;

error_chain!{
    foreign_links{
        ParseError(url::ParseError);
    }
}

pub fn test() -> Result<()>{
    let s = "gmvpn://10.0.90.211:1080/123123";    
    let url = Url::parse(s)?;
    println!("{}, {:?}, {:?}", url.scheme(), url.host(), url.port());
    Ok(())
}