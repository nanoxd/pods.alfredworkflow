extern crate alfred;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate quicli;
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
  let pods = response.into_allocation();

  match pods {
    Some(pods) => Ok(pods.pods()),
    None => Err(format_err!("Invalid Response")),
  }
}

fn pod_to_alfred_item(pods: Vec<Pod>, query: &str) -> io::Result<()> {
  let items: Vec<_> = pods
    .into_iter()
    .map(|pod| {
      ItemBuilder::new(pod.title())
        .text_copy(pod.stanza())
        .arg(pod.url())
        .valid(true)
        .subtitle(pod.summary)
        .subtitle_mod(Modifier::Option, format!("Open Repo: {}", pod.source.git))
        .arg_mod(Modifier::Option, pod.source.git)
        .into_item()
    })
    .collect();

  if items.is_empty() {
    no_items(query)
  } else {
    alfred::json::write_items(io::stdout(), &items)
  }
}

fn no_items(query: &str) -> io::Result<()> {
  let item = ItemBuilder::new("No Pods Found")
    .subtitle("Open CocoaPods Search?")
    .arg(format!("https://cocoapods.org/?q={}", query))
    .into_item();

  alfred::json::write_items(io::stdout(), &[item])
}

fn placeholder_item() -> io::Result<()> {
  let item = ItemBuilder::new("Search for your favorite pods")
    .arg("https://cocoapods.org")
    .into_item();

  alfred::json::write_items(io::stdout(), &[item])
}

#[derive(Debug, StructOpt)]
struct Cli {
  query: String,
}

main!(|args: Cli| {
  let query = &args.query;

  if query.is_empty() {
    placeholder_item()?
  } else {
    let request = search_pods(query);

    match request {
      Ok(req) => pod_to_alfred_item(req, query)?,
      Err(_) => no_items(query)?,
    };
  }
});
