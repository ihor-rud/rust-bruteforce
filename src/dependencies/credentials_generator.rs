use std::collections::HashMap;

use genawaiter::rc::gen;
use genawaiter::yield_;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

use crate::bruteforce::credentials::generator::Generator;
use crate::bruteforce::credentials::models::Credentials;

// Concreate generator for login and password pairs that uses commong login/password words and alphabet of similar characters
pub(crate) struct CredentialsGenerator<'a> {
    pub(crate) login_words: &'a [String],
    pub(crate) password_words: &'a [String],
    pub(crate) similar_characters: &'a HashMap<String, Vec<String>>,
}

impl<'a> Generator for CredentialsGenerator<'a> {
    fn generate(&self) -> Box<dyn Iterator<Item = Credentials> + '_> {
        Box::new(
            gen!({
                for login in self.login_words {
                    for password in self.password_words {
                        let login_graphemes = login.graphemes(true);

                        for similar_login in generate_all_similar_worlds(login_graphemes, self.similar_characters) {
                            let password_graphemes = password.graphemes(true);

                            for similar_password in
                                generate_all_similar_worlds(password_graphemes, self.similar_characters)
                            {
                                yield_!(Credentials {
                                    login: similar_login.clone(),
                                    password: similar_password,
                                });
                            }
                        }
                    }
                }
            })
            .into_iter(),
        )
    }
}

// Recursive algorithm that generates all possible similar world for a given sequence of characters
fn generate_all_similar_worlds<'a>(
    mut graphemes: Graphemes<'a>,
    similar_characters: &'a HashMap<String, Vec<String>>,
) -> Box<dyn Iterator<Item = String> + 'a> {
    let grapheme = match graphemes.next() {
        None => return Box::new(std::iter::once(String::default())),
        Some(grapheme) => grapheme,
    };

    let similar_graphemes = similar_characters.get(grapheme);

    match similar_graphemes {
        None => Box::new(
            gen!({
                for similar_tail in generate_all_similar_worlds(graphemes, similar_characters) {
                    yield_!(format!("{}{}", grapheme, similar_tail));
                }
            })
            .into_iter(),
        ),
        Some(similar_graphemes) => Box::new(
            gen!({
                for similar_tail in generate_all_similar_worlds(graphemes, similar_characters) {
                    for similar_grapheme in similar_graphemes {
                        yield_!(format!("{}{}", similar_grapheme, similar_tail));
                    }
                }
            })
            .into_iter(),
        ),
    }
}
