use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

pub fn test() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    // println!("{:?}", key_value);
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature = hmac::sign(&key, message.as_bytes());
    println!("{:?}", signature);

    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}
