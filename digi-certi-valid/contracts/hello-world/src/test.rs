#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn issue_verify_revoke_certificate() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.init(&admin);

    let cert_id = String::from_str(&env, "CERT-2026-001");
    let holder_name = String::from_str(&env, "Alice Doe");
    let course_name = String::from_str(&env, "Blockchain Fundamentals");
    let issuer = String::from_str(&env, "Stellar Academy");
    let issued_at = 1_742_348_800_u64;

    client.issue_certificate(&cert_id, &holder_name, &course_name, &issuer, &issued_at);

    let cert = client
        .get_certificate(&cert_id)
        .expect("certificate should exist after issue");

    assert_eq!(cert.holder_name, holder_name);
    assert_eq!(cert.course_name, course_name);
    assert_eq!(cert.issuer, issuer);
    assert_eq!(cert.issued_at, issued_at);
    assert!(cert.is_valid);
    assert!(client.is_certificate_valid(&cert_id));

    client.revoke_certificate(&cert_id);
    assert!(!client.is_certificate_valid(&cert_id));

    let admin_on_chain = client.get_admin();
    assert_eq!(admin_on_chain, admin);
}
