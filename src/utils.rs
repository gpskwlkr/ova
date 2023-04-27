use anyhow::{anyhow, Result};
use totp_rs::{Algorithm, Secret, TOTP};

pub fn get_2fa_code(key: &str) -> Result<String> {
    if key.trim().is_empty() {
        return Err(anyhow!("Key cannot be empty"));
    }

    let secret = Secret::Encoded(key.to_string());
    let totp = TOTP::new(Algorithm::SHA1, 6, 1, 30, secret.to_bytes().unwrap()).unwrap();
    let code = totp.generate_current()?;
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_2fa_code_with_empty_key() {
        let key = "";
        let code = get_2fa_code(key);
        assert!(code.is_err());
    }
}
