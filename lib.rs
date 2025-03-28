use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use ic_cdk::{query,update};

// Certificate structure matching your frontend model
#[derive(CandidType,Serialize, Clone, Debug)]
struct Certificate {
    id: String,
    name: String,
    course: String,
    date: String,
    revoked: bool,
}

// Store certificates in memory
thread_local! {
    static CERTIFICATES: RefCell<HashMap<String, Certificate>> = RefCell::new(HashMap::new());
}

// Issue a new certificate
#[update]
fn issue_certificate(name: String, course: String, date: String) -> Certificate {
    let id = format!("CERT-{}", ic_cdk::api::time());
    
    let certificate = Certificate {
        id: id.clone(),
        name,
        course,
        date,
        revoked: false,
    };
    
    CERTIFICATES.with(|certificates| {
        certificates.borrow_mut().insert(id.clone(), certificate.clone());
    });
    
    certificate
}

// Verify a certificate by ID
#[query]
fn verify_certificate(certificate_id: String) -> bool {
    CERTIFICATES.with(|certificates| {
        match certificates.borrow().get(&certificate_id) {
            Some(cert) => !cert.revoked,
            None => false,
        }
    })
}

// Revoke a certificate
#[update]
fn revoke_certificate(certificate_id: String) -> Result<Certificate, String> {
    CERTIFICATES.with(|certificates| {
        let mut certs = certificates.borrow_mut();
        
        match certs.get_mut(&certificate_id) {
            Some(cert) => {
                cert.revoked = true;
                Ok(cert.clone())
            },
            None => Err("Certificate not found".to_string()),
        }
    })
}

// Get all certificates (for the list display)
#[query]
fn get_all_certificates() -> Vec<Certificate> {
    CERTIFICATES.with(|certificates| {
        certificates.borrow().values().cloned().collect()
    })
}

// Get a specific certificate by ID
#[query]
fn get_certificate(certificate_id: String) -> Option<Certificate> {
    CERTIFICATES.with(|certificates| {
        certificates.borrow().get(&certificate_id).cloned()
    })
}

// Export Candid interface
ic_cdk::export_candid!();