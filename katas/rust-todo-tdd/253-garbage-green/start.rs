use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{PasswordHash as PhcString, SaltString, rand_core::OsRng},
};
use domain::identity::{HashingFailed, Password, PasswordHash};

pub fn hash_password(password: &Password) -> Result<PasswordHash, HashingFailed> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .map(|phc| PasswordHash::new(phc.to_string()))
        .map_err(|_| HashingFailed)
}

pub fn verify_password(candidate: &str, hash: &PasswordHash) -> bool {
    let parsed = PhcString::new(hash.as_str()).expect("a valid PHC string");
    Argon2::default()
        .verify_password(candidate.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pw(raw: &str) -> Password {
        Password::parse(raw).unwrap()
    }

    #[test]
    fn a_hash_verifies_the_original_password_only() {
        let hash = hash_password(&pw("correct horse battery")).unwrap();
        assert!(hash.as_str().starts_with("$argon2id$"));
        assert!(verify_password("correct horse battery", &hash));
        assert!(!verify_password("wrong horse battery", &hash));
    }

    #[test]
    fn the_same_password_hashes_differently_each_time() {
        let (a, b) = (
            hash_password(&pw("correct horse battery")).unwrap(),
            hash_password(&pw("correct horse battery")).unwrap(),
        );
        assert_ne!(a, b);
    }

    #[test]
    fn a_garbage_hash_never_verifies() {
        assert!(!verify_password(
            "anything",
            &PasswordHash::new("garbage".into())
        ));
    }
}
