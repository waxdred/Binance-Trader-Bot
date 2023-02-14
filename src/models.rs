use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config{
    pub webhook: String,
    pub delai: u64,
    pub url: Vec<String>,
    pub custom: Custom,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub title: String,
    pub description: String,
    pub username: String,
    pub thumbnail_url: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub author: String,
    pub content: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Uid{
    #[serde(rename = "encryptedUid")]
    pub encrypted_uid: String
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoUid {
    pub code: String,
    pub message: Value,
    pub message_detail: Value,
    pub data: Data,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoUids {
    pub data: InfoUid,
    pub uid: String,
}
impl InfoUids {
    pub fn new(info: InfoUid, uid: &String) -> InfoUids {
        InfoUids {
            data: info,
            uid: uid.to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub nick_name: String,
    pub user_photo_url: String,
    pub position_shared: bool,
    pub delivery_position_shared: bool,
    pub following_count: i64,
    pub follower_count: i64,
    pub twitter_url: Value,
    pub introduction: String,
    pub tw_shared: bool,
    pub is_tw_trader: bool,
    pub open_id: Value,
    pub portfolio_id: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub code: String,
    pub message: Value,
    pub message_detail: Value,
    pub data: DataTrade,
    pub success: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataTrade {
    pub other_position_ret_list: Vec<OtherPositionRetList>,
    pub update_time: Vec<i64>,
    pub update_time_stamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherPositionRetList {
    pub symbol: String,
    pub entry_price: f64,
    pub mark_price: f64,
    pub pnl: f64,
    pub roe: f64,
    pub update_time: Vec<i64>,
    pub amount: f64,
    pub update_time_stamp: i64,
    pub yellow: bool,
    pub trade_before: bool,
    pub leverage: i64,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct OtherPositionRetListBool {
    pub data: OtherPositionRetList,
    pub trade: bool,
}

impl OtherPositionRetListBool {
    pub fn new(other:OtherPositionRetList, b:bool)->OtherPositionRetListBool{
        OtherPositionRetListBool{
            data: other,
            trade: b,
        }
    }
    pub fn change_trade(&mut self, trade: bool){
        self.trade = trade;
    }
}
