use std::fmt::format;

use reqwest::header::HeaderMap;
use serde_json::json;
use crate::models;
use crate::proxy;
extern crate fake_useragent;
// https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT
//

pub async fn current_price(symbol:&str)->Result<models::Symbol, reqwest::Error>{
    let list_proxy = proxy::Proxy::new().await;
    let ip = list_proxy.random();
    let conn = format!("http://{}:{}", ip.ip, ip.port);
    let proxy = reqwest::Proxy::http(conn)?;
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    let resp = match client.get(url).send().await {
        Ok(resp) => {
            resp.json::<models::Symbol>().await
        },
        Err(err) => {
            return Err(err);
        }
    };
    resp
}

pub async fn post_requet(uid: String)->Result<models::InfoUid, reqwest::Error>{
    let list_proxy = proxy::Proxy::new().await;
    let ip = list_proxy.random();
    let conn = format!("http://{}:{}", ip.ip, ip.port);
    let proxy = reqwest::Proxy::http(conn)?;
    let url = "https://www.binance.com/bapi/futures/v2/public/future/leaderboard/getOtherLeaderboardBaseInfo";
    let data = models::Uid{encrypted_uid: uid};
    let client = reqwest::Client::builder().proxy(proxy).build()?;
    let resp = match client.post(url)
        .json(&data).send().await {
        Ok(resp) => {
            resp.json::<models::InfoUid>().await
        },
        Err(err) => {
            return Err(err);
        }
    };
    resp
}

pub async fn post_get_trade(uid: String)->Result<models::Trade, reqwest::Error>{
    let list_proxy = proxy::Proxy::new().await;
    let ip = list_proxy.random();
    let conn = format!("http://{}:{}", ip.ip, ip.port);
    let proxy = reqwest::Proxy::http(conn)?;
    let url = "https://www.binance.com/bapi/futures/v1/public/future/leaderboard/getOtherPosition";
    let client = reqwest::Client::builder().proxy(proxy).build()?;
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
            return Err(err);
        }
    };
    resp
}

