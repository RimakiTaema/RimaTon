use crate::reqwest::*;
use crate::logger::*;

const API_URL: &str = "https://api.exaroton.com/v1";

pub fn auth_check(token: &str) {
    info("API", "Auth Checking Request Recived")
}