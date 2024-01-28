// Custom sanitization function for general strings (trimming)
pub fn _trim_string(input: &str) -> String {
    input.trim().to_string()
}

// Custom sanitization function for email addresses
pub fn _sanitize_email(email: &str) -> String {
    email.trim().to_lowercase().to_string()
}
