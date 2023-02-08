use crate::{models::{self, InfoUid}};
use webhook::client::{WebhookClient, WebhookResult};
use webhook::models::NonLinkButtonStyle;

pub async fn send_webhook(pos: models::OtherPositionRetList, configs: models::Config, trader:InfoUid, title: &str){
    let client: WebhookClient = WebhookClient::new(&configs.webhook);
    client.send(|message| message
        .content("@everyone")
        .username(&trader.data.nick_name)
        .avatar_url(&trader.data.user_photo_url)
        .embed(|embed| embed
            .title(title)
            .description(&title)
            // .thumbnail(IMAGE_URL)
            .author(&trader.data.nick_name, Some(String::from(&trader.data.user_photo_url)), Some(String::from(&trader.data.user_photo_url)))
            .field("Prix d'entrée:", "> {entry}", false)
            .field("Prix du marché:", "> $", false)
            .field("Long ou Short:", "> ", false)
            .field("Taille:", "> ", false)
            .field("Levier", "val", false)
            )).await;
}
