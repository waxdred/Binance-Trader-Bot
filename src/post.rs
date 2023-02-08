use reqwest::header::HeaderMap;
use serde_json::json;
use crate::models;
extern crate fake_useragent;

pub async fn post_requet(uid: String)->Result<models::InfoUid, reqwest::Error>{
    let url = "https://www.binance.com/bapi/futures/v2/public/future/leaderboard/getOtherLeaderboardBaseInfo";
    let data = models::Uid{encrypted_uid: uid};
    let client = reqwest::Client::new();
    let resp = match client.post(url)
        .json(&data).send().await {
        Ok(resp) => resp.json::<models::InfoUid>().await,
        Err(err) => panic!("Error: {}", err)
    };
    resp
}

pub async fn post_get_trade(uid: String)->Result<models::Trade, reqwest::Error>{
    let url = "https://www.binance.com/bapi/futures/v1/public/future/leaderboard/getOtherPosition";
    let client = reqwest::Client::new();
    let body = json!({
        "encryptedUid": uid,
        "tradeType": "PERPETUAL"
    });
    use fake_useragent::UserAgents;
    let user_agents = UserAgents::new();
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", user_agents.random().parse().unwrap());
    headers.insert("Accept-Encoding", "*".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    let resp = match client.post(url)
        .headers(headers).json(&body).send().await{
        Ok(resp) => resp.json::<models::Trade>().await,
        Err(err) => {
            panic!("Error: {}", err)}
    };
    println!("{:#?}", resp);
    resp
}

