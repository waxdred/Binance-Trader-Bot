use crate::{models};

pub async fn send_webhook(pos: models::OtherPositionRetList, configs: models::Config){
    println!("{:#?}\n{:#?}", pos, configs);
}
