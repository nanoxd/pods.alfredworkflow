extern crate alfred;
#[macro_use]
extern crate quicli;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
use quicli::prelude::*;
use models::{Pod, SearchResponse};

mod models;

fn search_pods(query: &str) -> Result<Vec<Pod>> {
  let url: &str = &format!("https://search.cocoapods.org/api/v1/pods.picky.hash.json?query={}&ids=10&offset=0&sort=quality", query);
  let mut response: SearchResponse = reqwest::get(url)?.json()?;
  let pods = response.into_allocation().pods();

  Ok(pods)
}

#[derive(Debug, StructOpt)]
struct Cli {
  query: String,
}

main!(|args: Cli| {
  let query = &args.query;
  let request = search_pods(query)?;

  println!("{:?}", request)
});
