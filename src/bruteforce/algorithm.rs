use super::credentials::checker::{CheckResult, Checker};
use super::credentials::generator::Generator;
use super::credentials::models::Credentials;

pub(crate) async fn brute_force(checker: &dyn Checker, credentials_generator: &dyn Generator) -> Option<Credentials> {
    for credentials in credentials_generator.generate() {
        let result = checker.check(credentials).await;

        if let CheckResult::Match(credentials) = result {
            return Some(credentials);
        }
    }

    None
}
