mod example_a;
mod example_b;
mod example_c;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error> >{
    // example_a::test()?;
    // example_b::test()?;
    example_c::test()?;
    Ok(())
}
