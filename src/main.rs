extern crate alfred;
#[macro_use]
extern crate quicli;
extern crate reqwest;
use quicli::prelude::*;

fn make_request(query: &str) -> Result<String> {
  let url: &str = &format!("https://search.cocoapods.org/api/v1/pods.picky.hash.json?query={}&ids=10&offset=0&sort=quality", query);
  let response = reqwest::get(url)?.text()?;

  println!("{}", response);
  Ok(response)
}

#[derive(Debug, StructOpt)]
struct Cli {
  query: String,
}

main!(|args: Cli| {
  let query = &args.query;
  make_request(query);
});
