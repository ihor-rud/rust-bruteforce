use super::models::Credentials;

pub(crate) trait Generator {
    fn generate(&self) -> Box<dyn Iterator<Item = Credentials> + '_>;
}
