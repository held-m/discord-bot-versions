use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::{Deserialize, Serialize};
use std::str;

use reqwest::{header::USER_AGENT, Client};

#[derive(Serialize, Deserialize, Debug)]
struct Release {
    url: String,
    tag_name: String,
}

#[command]
pub async fn release(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let lang_input = args.single::<String>().unwrap_or("".to_string());

    if lang_input != "" {
        msg.channel_id
            .say(&ctx.http, lang_fabric(lang_input).await)
            .await?;
        // lastest_vesrion = lang_fabric(lang_input).await;
    } else {
        let channel_name = ctx
            .http
            .get_channel(*msg.channel_id.as_u64())
            .await
            .expect("err")
            .guild()
            .expect("rr")
            .name;

        msg.channel_id
            .say(&ctx.http, lang_fabric(channel_name).await)
            .await?;
    }

    Ok(())
}

async fn lang_fabric(lang: String) -> String {
    match lang.as_str() {
        "rust" | "rust-lang" | "rustlang" => rust().await,
        "go" | "golang" | "go-lang" => go().await,
        _ => "Please check the name of a language or the name of a channel".to_string(),
    }
}

async fn rust() -> String {
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

async fn go() -> String {
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
