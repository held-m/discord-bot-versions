use reqwest::{header::USER_AGENT, Client};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Release {
    url: String,
    tag_name: String,
}

pub async fn parse() -> String {
    let client = Client::new();
    let res = client
        .get("https://api.github.com/repos/rust-lang/rust/releases/latest")
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await
        .expect("answer from server")
        .json::<Release>()
        .await
        .expect("convert json to struct");

    println!("{:?}", res.tag_name);
    return res.tag_name;
}
