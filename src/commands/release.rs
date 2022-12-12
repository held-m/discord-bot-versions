use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use super::super::parsers::{go, rust};

#[command]
pub async fn v(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
        "rust" | "rust-lang" | "rustlang" => rust::parse().await,
        "go" | "golang" | "go-lang" => go::parse().await,
        _ => "Please check the name of a language or the name of a channel".to_string(),
    }
}
