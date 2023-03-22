use bitflags::bitflags;
use std::fmt;

bitflags! {
    struct MyFlags: u32{
        const FLAG_A    = 0b00000001;
        const FLAG_B    = 0b00000010;
        const FLAG_C    = 0b00000100;
        const FLAG_ABC  = Self::FLAG_A.bits() | Self::FLAG_B.bits() | Self::FLAG_C.bits();
    }
}

impl MyFlags {
    pub fn clear(&mut self) -> &mut MyFlags {
        *self.0.bits_mut() = 0;
        self
    }
}

impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}

pub fn test(){
    let mut e1 = MyFlags::FLAG_A | MyFlags::FLAG_B;
    println!("{}", e1);
    e1.clear();
    println!("{}", e1);
    e1 = MyFlags::FLAG_ABC;
    println!("{}", e1);
}