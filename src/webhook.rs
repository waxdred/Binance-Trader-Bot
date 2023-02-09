use std::{error::Error, collections::HashMap};

use crate::models::{self, InfoUid};
use webhook::client::WebhookClient;

async fn map_webhook(custom: models::Custom, trader:InfoUid, title: &str)-> HashMap<String, String>{
    let mut hook = HashMap::new();
    if !custom.title.is_empty(){
        hook.insert(String::from("title"),  custom.title);
    }else{
        hook.insert(String::from("title"),  title.to_string());
    }
    if !custom.description.is_empty(){
        hook.insert(String::from("description"),  custom.description);
    }else{
        hook.insert(String::from("description"),  String::from("trade from binance leaderboard"));
    }
    if !custom.username.is_empty(){
        hook.insert(String::from("username"),  custom.username);
    }else{
        hook.insert(String::from("username"),  trader.data.nick_name.clone());
    }
    if !custom.thumbnail_url.is_empty(){
        hook.insert(String::from("thumbnail"),  custom.thumbnail_url);
    }else{
        hook.insert(String::from("thumbnail"),  trader.data.user_photo_url.clone());
    }
    if !custom.avatar_url.is_empty(){
        hook.insert(String::from("avatar_url"),  custom.avatar_url);
    }else{
        hook.insert(String::from("avatar_url"),  trader.data.user_photo_url.clone());
    }
    if !custom.author.is_empty(){
        hook.insert(String::from("author"),  custom.author);
    }else{
        hook.insert(String::from("author"),  trader.data.nick_name);
    }
    if !custom.content.is_empty(){
        hook.insert(String::from("content"),  custom.content);
    }else{
        hook.insert(String::from("content"),  String::from("@everyone"));
    }
    hook
}

pub async fn send_webhook(pos: models::OtherPositionRetList, configs: models::Config, trader:InfoUid, title: &str)-> Result<(), Box<dyn Error + Send + Sync>>{
    let client: WebhookClient = WebhookClient::new(&configs.webhook);
    let field = map_webhook(configs.custom, trader.clone(), title).await;
    let mut side:String = String::new();
    let mut date_value:String = String::new();
    date_value = "".to_string(); 

    println!("{:#?}", field);
    if pos.amount <= 0.0{
        side = "🔴 Sell".to_string();
    }else{
        side = "🟢 Buy".to_string();
    }
    if !pos.update_time.is_empty(){
        date_value = format!("{}/{}/{} at {}:{}:{}", pos.update_time[2], pos.update_time[1], pos.update_time[0], pos.update_time[3], pos.update_time[4], pos.update_time[5])
    }
    match client.send(|message| message
        .content("@everyone")
        .username(&field["username"])
        .avatar_url(&field["avatar_url"])
        .embed(|embed| embed
            .title(&field["title"])
            .description(&field["description"])
            .thumbnail(&field["thumbnail"].clone())
            .author(&field["author"], Some(field["thumbnail"].clone()), Some(field["thumbnail"].clone()))
            .field("Date", &date_value, false)
            .field("Entry price:", &format!("{} $", pos.entry_price), false)
            .field("Market price:", &format!("{} $", pos.mark_price), false)
            .field("Long ou Short:", &side, false)
            .field("Taille:", &format!("{}", pos.amount), false)
            .field("Leverage", &format!("{}", pos.leverage), false)
            )).await{
        Ok(ret) =>ret,
        Err(err) =>{
            print!("{:#?}", err);
            return Err(err);
        }
    };
    Ok(())
}
