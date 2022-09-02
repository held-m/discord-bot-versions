use reqwest::{header::USER_AGENT, Client};

pub async fn parse() -> String {
    let client = Client::new();
    let res = client
        .get("https://go.dev/VERSION?m=text")
        .header(USER_AGENT, "My Rust Program 1.0")
        .send()
        .await
        .expect("answer from server")
        .text()
        .await
        .expect("go version");

    println!("{:?}", res);
    return res;
}
