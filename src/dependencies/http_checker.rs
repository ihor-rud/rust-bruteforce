use async_trait::async_trait;
use reqwest::Client;

use crate::bruteforce::credentials::checker::{CheckResult, Checker};
use crate::bruteforce::credentials::models::Credentials;

pub(crate) struct HttpChecker {
    pub(crate) client: Client,
    pub(crate) url: String,
}

#[async_trait]
impl Checker for HttpChecker {
    async fn check(&self, credentials: Credentials) -> CheckResult {
        let resp = self
            .client
            .get(&self.url)
            .basic_auth(&credentials.login, Some(&credentials.password))
            .send()
            .await;

        match resp {
            Ok(responce) if responce.status().is_success() => CheckResult::Match(credentials),
            _ => CheckResult::Fail,
        }
    }
}
