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
use alfred::{ItemBuilder, Modifier};
use std::io;

mod models;

fn search_pods(query: &str) -> Result<Vec<Pod>> {
  let url: &str = &format!("https://search.cocoapods.org/api/v1/pods.picky.hash.json?query={}&ids=10&offset=0&sort=quality", query);
  let mut response: SearchResponse = reqwest::get(url)?.json()?;
  let pods = response.into_allocation().pods();

  Ok(pods)
}

fn pod_to_alfred_item(pods: Vec<Pod>) -> io::Result<()> {
  let items: Vec<_> = pods
    .into_iter()
    .map(|pod| {
      ItemBuilder::new(format!("{}", pod.id))
        .text_copy(pod.stanza())
        .arg(pod.url())
        .valid(true)
        .subtitle(pod.summary)
        .subtitle_mod(Modifier::Option, format!("Open Repo: {}", pod.source.git))
        .arg_mod(Modifier::Option, pod.source.git)
        .into_item()
    })
    .collect();

  alfred::json::write_items(io::stdout(), &items)
}

#[derive(Debug, StructOpt)]
struct Cli {
  query: String,
}

main!(|args: Cli| {
  let query = &args.query;
  let request = search_pods(query)?;
  pod_to_alfred_item(request)?
});
