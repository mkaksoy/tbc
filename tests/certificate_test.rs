use tbc::{self, utils::certificate::*};
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;

#[test]
fn random_salt_hash() {
    assert_ne!(
        generate_hash("password", None).unwrap(),
        generate_hash("password", None).unwrap()
    );
}

#[test]
fn constant_salt_hash() {
    let salt = generate_random_salt().expect("Error generating salt");
    let hash = generate_hash("password", Some(salt.to_string())).unwrap();
    let hash2 = generate_hash("password", Some(salt.to_string())).unwrap();
    assert_eq!(hash, hash2);
}

#[test]
fn test_generate_random_salt() {
    let salt = generate_random_salt();
    assert!(salt.is_ok());
    let salt_str = salt.unwrap();
    let decoded = STANDARD_NO_PAD.decode(&salt_str);
    assert!(decoded.is_ok());
    assert_eq!(decoded.unwrap().len(), 16);
}

#[test]
fn test_generate_hash_and_verify_consistency() {
    let value = "test_value";
    let salt = generate_random_salt().unwrap();

    let hash1 = generate_hash(value, Some(salt.clone())).unwrap();
    let hash2 = generate_hash(value, Some(salt)).unwrap();

    assert_eq!(hash1, hash2);
}

#[test]
fn test_generate_hash_different_salts_gives_different_hashes() {
    let value = "test_value";
    let hash1 = generate_hash(value, None).unwrap();
    let hash2 = generate_hash(value, None).unwrap();
    assert_ne!(hash1, hash2);
}

#[test]
fn test_sign_and_verify_certificate() {
    let content = "certificate_data";
    let password = "secret_password";

    let signature = sign_certificate(content, password);
    assert!(verify_signature(content, &signature, password));
}

#[test]
fn test_verify_certificate_with_wrong_password() {
    let content = "certificate_data";
    let correct_password = "secret_password";
    let wrong_password = "wrong_password";

    let signature = sign_certificate(content, correct_password);
    assert!(!verify_signature(content, &signature, wrong_password));
}

#[test]
fn test_verify_certificate_with_tampered_content() {
    let content = "certificate_data";
    let password = "secret_password";
    let tampered_content = "certificate_datb";

    let signature = sign_certificate(content, password);
    assert!(!verify_signature(tampered_content, &signature, password));
}
