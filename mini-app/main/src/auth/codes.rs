use std::{sync::Arc, time::Instant};

use bson::oid::ObjectId;
use model::user::sanitize_phone;
use rand::Rng as _;

#[derive(Default, Clone)]
pub struct Codes {
    code_requests: Arc<dashmap::DashMap<PhoneNumber, VerificationCode>>,
}

impl Codes {
    pub fn get_code(&self, phone: &PhoneNumber) -> Option<VerificationCode> {
        self.code_requests.get(phone).map(|v| v.clone())
    }

    pub fn generate_code(&self, user: model::user::User) -> VerificationCode {
        let code = VerificationCode::new(user.id);
        self.code_requests
            .insert(PhoneNumber::new(user.phone.unwrap()), code.clone());
        code
    }

    pub fn auth(&self, phone: &PhoneNumber, verification_code: String) -> AuthResult {
        if let Some(mut code) = self.code_requests.get_mut(phone) {
            code.attempts = code.attempts.saturating_sub(1);
            if code.attempts == 0 {
                return AuthResult::TooManyAttempts;
            }
            if !code.is_valid() {
                return AuthResult::Expired;
            }
            if code.code != verification_code {
                return AuthResult::InvalidCode;
            }
            AuthResult::Success(code.user_id)
        } else {
            AuthResult::InvalidPhone
        }
    }

    pub fn gc(&self) {
        self.code_requests.retain(|_, v| v.is_valid());
    }
}

pub enum AuthResult {
    Success(ObjectId),
    InvalidPhone,
    InvalidCode,
    Expired,
    TooManyAttempts,
}

#[derive(Clone, Debug)]
pub struct VerificationCode {
    pub code: String,
    pub user_id: ObjectId,
    pub expires_at: Instant,
    pub attempts: u8,
}

impl VerificationCode {
    pub fn new(user_id: ObjectId) -> Self {
        let code = generate_code();
        let expires_at = Instant::now() + std::time::Duration::from_secs(60);
        VerificationCode {
            code,
            user_id,
            expires_at,
            attempts: 3,
        }
    }

    pub fn left_time(&self) -> u64 {
        let now = Instant::now();
        if self.expires_at < now {
            return 0;
        }
        (self.expires_at - now).as_secs()
    }

    pub fn is_valid(&self) -> bool {
        self.expires_at > Instant::now()
    }
}

fn generate_code() -> String {
    let mut rng = rand::thread_rng();
    let code: u32 = rng.gen_range(1000..9999);
    code.to_string()
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct PhoneNumber {
    number: String,
}

impl PhoneNumber {
    pub fn new(number: String) -> Self {
        PhoneNumber {
            number: sanitize_phone(&number),
        }
    }
}

impl AsRef<str> for PhoneNumber {
    fn as_ref(&self) -> &str {
        &self.number
    }
}

impl From<String> for PhoneNumber {
    fn from(number: String) -> Self {
        PhoneNumber::new(number)
    }
}

impl From<&str> for PhoneNumber {
    fn from(number: &str) -> Self {
        PhoneNumber {
            number: sanitize_phone(number),
        }
    }
}
