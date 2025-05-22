use tbc::{self, utils::certificate};

#[test]
fn random_salt_hash() {
    assert_ne!(certificate::generate_hash("password", None).unwrap(), certificate::generate_hash("password", None).unwrap());
}

#[test]
fn constant_salt_hash() {
    let salt = "mF3f5kU1KkLQeYrRfQdZmA";
    let hash = certificate::generate_hash("password", Some(salt.to_string())).unwrap();
    let hash2 = certificate::generate_hash("password", Some(salt.to_string())).unwrap();
    assert_eq!(hash, hash2);
}