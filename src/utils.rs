use anyhow::Result;
use totp_rs::{Algorithm, Secret, TOTP};

pub fn get_2fa_code(key: &str) -> Result<String> {
    let secret = Secret::Encoded(key.to_string());
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.to_bytes().unwrap()).unwrap();
    let code = totp.generate_current()?;
    Ok(code.to_string())
}
