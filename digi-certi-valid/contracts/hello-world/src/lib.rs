#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Certificate(String),
}

#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    pub cert_id: String,
    pub holder_name: String,
    pub course_name: String,
    pub issuer: String,
    pub issued_at: u64,
    pub is_valid: bool,
}

#[contractimpl]
impl Contract {
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn issue_certificate(
        env: Env,
        cert_id: String,
        holder_name: String,
        course_name: String,
        issuer: String,
        issued_at: u64,
    ) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("contract not initialized"));
        admin.require_auth();

        let key = DataKey::Certificate(cert_id.clone());
        if env.storage().persistent().has(&key) {
            panic!("certificate already exists");
        }

        let cert = Certificate {
            cert_id,
            holder_name,
            course_name,
            issuer,
            issued_at,
            is_valid: true,
        };

        env.storage().persistent().set(&key, &cert);
    }

    pub fn revoke_certificate(env: Env, cert_id: String) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("contract not initialized"));
        admin.require_auth();

        let key = DataKey::Certificate(cert_id);
        let mut cert: Certificate = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("certificate not found"));

        cert.is_valid = false;
        env.storage().persistent().set(&key, &cert);
    }

    pub fn get_certificate(env: Env, cert_id: String) -> Option<Certificate> {
        let key = DataKey::Certificate(cert_id);
        env.storage().persistent().get(&key)
    }

    pub fn is_certificate_valid(env: Env, cert_id: String) -> bool {
        let key = DataKey::Certificate(cert_id);

        if let Some(cert) = env.storage().persistent().get::<DataKey, Certificate>(&key) {
            cert.is_valid
        } else {
            false
        }
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic!("contract not initialized"))
    }
}

mod test;
