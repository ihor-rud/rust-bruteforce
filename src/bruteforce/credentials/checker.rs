use async_trait::async_trait;

use super::models::Credentials;

pub(crate) enum CheckResult {
    Fail,
    Match(Credentials),
}

#[async_trait]
pub(crate) trait Checker {
    async fn check(&self, credentials: Credentials) -> CheckResult;
}
