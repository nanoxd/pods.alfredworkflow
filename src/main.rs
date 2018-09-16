extern crate alfred;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate quicli;
#[macro_use]
extern crate serde_derive;
use alfred::{ItemBuilder, Modifier};
use models::{AlgoliaResponse, Pod};
use quicli::prelude::*;
use std::collections::HashMap;
use std::io;

mod models;

const ALGOLIA_APPLICATION_ID: &'static str = "WBHHAMHYNM";
const ALGOLIA_API_KEY: &'static str = "4f7544ca8701f9bf2a4e55daff1b09e9";

fn search_pods_v2(query: &str) -> Result<Vec<Pod>> {
    let url: &str = &format!("https://wbhhamhynm-3.algolianet.com/1/indexes/cocoapods/query?x-algolia-application-id={}&x-algolia-api-key={}", ALGOLIA_APPLICATION_ID, ALGOLIA_API_KEY);
    let client = reqwest::Client::new();

    let mut map = HashMap::new();
    let q = format!("query={}", query);
    map.insert("params", q);

    let response: AlgoliaResponse = client.post(url).json(&map).send()?.json()?;
    Ok(response.hits)
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
        }).collect();

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
        let request = search_pods_v2(query);

        match request {
            Ok(req) => pod_to_alfred_item(req, query)?,
            Err(_) => no_items(query)?,
        };
    }
});
