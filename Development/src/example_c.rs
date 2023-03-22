use error_chain::error_chain;
use std::ffi::{CString, c_char};

error_chain! {
    foreign_links {
        NulError(::std::ffi::NulError);
        Io(::std::io::Error);
    }
}

extern {
    fn hello();
    fn greet(name: *const c_char);
    fn sum(x: i32, y: i32) -> i32;
}

pub fn test(){
    unsafe { hello() }
    let name = "test";
    let c_name = CString::new(name.to_string()).unwrap();
    unsafe { greet(c_name.as_ptr()) }
    
    println!("{}", unsafe { sum(1,100) });
}