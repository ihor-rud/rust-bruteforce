use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn load_words(filename: &String) -> Option<Vec<String>> {
    let file = BufReader::new(File::open(filename).ok()?);
    let lines: Result<_, _> = file.lines().collect();
    lines.ok()
}

pub(crate) fn load_alphabet(filename: &String) -> Option<HashMap<String, Vec<String>>> {
    Some(
        load_words(filename)?
            .iter()
            .flat_map(|line| {
                std::iter::zip(
                    line.graphemes(true),
                    std::iter::repeat_with(|| {
                        line.graphemes(true)
                            .map(|val| val.to_owned())
                            .collect()
                    }),
                )
            })
            .map(|(key, val)| (key.to_owned(), val))
            .collect(),
    )
}
