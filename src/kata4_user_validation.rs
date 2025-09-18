// ============================================
// Kata 4: User Validation System
// Focus: Newtype Pattern, Strong Typing, Validation Logic
// ============================================

use std::error::Error;
use std::fmt;
use regex::Regex;


#[derive(Debug, Clone, PartialEq)]
pub struct Username(String);

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

#[derive(Debug, Clone)]
pub struct Password(String);

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    UsernameTooShort,
    UsernameTooLong,
    UsernameInvalidCharacters,
    EmailInvalidFormat,
    PasswordTooWeak,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::UsernameTooShort => write!(f, "Username must be at least 3 characters"),
            ValidationError::UsernameTooLong => write!(f, "Username must be at most 20 characters"),
            ValidationError::UsernameInvalidCharacters => write!(f, "Username contains invalid characters"),
            ValidationError::EmailInvalidFormat => write!(f, "Invalid email format"),
            ValidationError::PasswordTooWeak => write!(f, "Password must be at least 8 characters with numbers and letters"),
        }
    }
}

impl Error for ValidationError {}

impl Username {
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 20;

    /// Create a new username with validation
    pub fn new(username: &str) -> Result<Self, ValidationError> {
        let trimmed = username.trim();
        
        if trimmed.len() < Self::MIN_LENGTH {
            return Err(ValidationError::UsernameTooShort);
        }
        
        if trimmed.len() > Self::MAX_LENGTH {
            return Err(ValidationError::UsernameTooLong);
        }
        
        if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(ValidationError::UsernameInvalidCharacters);
        }
        
        Ok(Username(trimmed.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Email {
    /// Create a new email address with basic validation
    pub fn new(email: &str) -> Result<Self, ValidationError> {
        let trimmed = email.trim().to_lowercase();
        
        // Simple email validation
        if !trimmed.contains('@') || !trimmed.contains('.') {
            return Err(ValidationError::EmailInvalidFormat);
        }
        
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(ValidationError::EmailInvalidFormat);
        }
        
        Ok(Email(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

impl Password {
    const MIN_LENGTH: usize = 8;

    /// Create a new password with strength validation
    pub fn new(password: &str) -> Result<Self, ValidationError> {
        if password.len() < Self::MIN_LENGTH {
            return Err(ValidationError::PasswordTooWeak);
        }
        
        let has_letter = password.chars().any(|c| c.is_alphabetic());
        let has_digit = password.chars().any(|c| c.is_numeric());
        
        if !has_letter || !has_digit {
            return Err(ValidationError::PasswordTooWeak);
        }
        
        Ok(Password(password.to_string()))
    }

    /// Verify if the password matches
    pub fn verify(&self, input: &str) -> bool {
        self.0 == input
    }
}

/// User account
pub struct UserAccount {
    username: Username,
    email: Email,
    password: Password,
}

impl UserAccount {
    pub fn new(username: Username, email: Email, password: Password) -> Self {
        Self { username, email, password }
    }

    pub fn username(&self) -> &Username {
        &self.username
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn verify_password(&self, password: &str) -> bool {
        self.password.verify(password)
    }
}

#[cfg(test)]
mod user_validation_tests {
    use super::*;

    #[test]
    fn test_username_validation() {
        assert!(Username::new("abc").is_ok());
        assert_eq!(Username::new("ab"), Err(ValidationError::UsernameTooShort));
        assert_eq!(Username::new("a".repeat(21).as_str()), Err(ValidationError::UsernameTooLong));
        assert_eq!(Username::new("user@name"), Err(ValidationError::UsernameInvalidCharacters));
    }

    #[test]
    fn test_email_validation() {
        assert!(Email::new("user@example.com").is_ok());
        assert_eq!(Email::new("invalid"), Err(ValidationError::EmailInvalidFormat));
    }

    #[test]
    fn test_password_validation() {
        assert!(Password::new("password123").is_ok());
        assert_eq!(Password::new("short"), Err(ValidationError::PasswordTooWeak));
        assert_eq!(Password::new("nodigits"), Err(ValidationError::PasswordTooWeak));
    }
}
