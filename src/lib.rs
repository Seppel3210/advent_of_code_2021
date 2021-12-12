#![feature(try_blocks)]
use reqwest::{blocking::Client, cookie::Jar};
use std::{error::Error, fs, sync::Arc};

const URL: &'static str = "https://adventofcode.com";
const TOKEN: &'static str = include_str!("../session_token.txt");

pub fn input(path: &str) -> String {
    let _ = fs::create_dir("./input");
    let file_path = format!("./input/{path}");
    fs::read_to_string(&file_path).unwrap_or_else(|_| {
        let res: Result<_, Box<dyn Error>> = try {
            let jar = Jar::default();
            let url = URL.parse()?;
            jar.add_cookie_str(&format!("session={TOKEN}"), &url);
            let client = Client::builder().cookie_provider(Arc::new(jar)).build()?;

            client
                .get(url.join(&format!("2021/day/{path}/input"))?)
                .send()?
                .text()?
        };
        let input = res.unwrap();
        fs::write(file_path, &input).unwrap();
        input
    })
}
