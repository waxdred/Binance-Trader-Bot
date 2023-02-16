#[cfg(debug_assertions)]
use chrono::Local;
use std::time::Duration;
use tokio::time::sleep;

use crate::{post, webhook, models};

pub async fn follow_trade(uid: String, configs: models::Config)->Result<(), reqwest::Error>{
    let mut history:Vec<models::OtherPositionRetList> = Vec::new();
    let trader = match post::post_requet(uid.clone()).await{
        Ok(trader)=>trader,
        Err(err)=> {
            return Err(err);
        }
    };
    loop {
        let current_time = Local::now();
        let time_str = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
        // add try catch
        let trade = match post::post_get_trade(uid.clone()).await{
            Ok(trade)=> trade,
            Err(_err)=> {
                continue;
            }
        };
        println!("{}\n{:#?}",time_str, trade);
        if !trade.data.other_position_ret_list.is_empty(){
            let mut tmp = trade.data.other_position_ret_list;
            if tmp.len() > history.len(){
                for t in tmp.iter(){
                    let mut check = true;
                    for h in history.iter(){
                        if t.update_time_stamp == h.update_time_stamp{
                            check = false;
                        }
                    }
                    if check{
                        //send to webhook new trade
                        if !configs.whitelist.is_empty() {
                            if configs.whitelist.iter().any(|x| x == &t.symbol.clone()){
                                history.insert(0, t.clone());
                                match webhook::send_webhook(t.clone(), configs.clone(),trader.clone(), "New Trade", true).await{
                                   Ok(_val) => (),
                                   Err(err)=>{
                                       println!("{}", err);
                                   }
                                };
                            }
                        }else if !configs.blacklist.is_empty() && configs.blacklist.contains(&t.symbol.clone()){
                            continue;
                        }else{
                            history.insert(0, t.clone());
                            match webhook::send_webhook(t.clone(), configs.clone(),trader.clone(), "New Trade", true).await{
                                Ok(_val) => (),
                                Err(err)=>{
                                    println!("{}", err);
                                }
                            };
                        }
                        sleep(Duration::from_secs(configs.delai)).await;
                        //add time sleep
                    }
                }
            } 
            if !history.is_empty(){
                let save = tmp.clone();
                // println!("tmp {:#?}", tmp.clone());
                history.retain(|x| !tmp.iter().any(|y| y.update_time_stamp == x.update_time_stamp && y.symbol == x.symbol));
                // println!("history {:#?}", history.clone());
                for h in history.iter(){
                    //close trade 
                    let mut close = h.to_owned();
                    let price = match post::current_price("BTCUSDT").await{
                        Ok(price)=> price,
                        Err(_err)=> {
                            continue;
                        }
                    };
                    close.set_price(&price.price);
                    match webhook::send_webhook(close, configs.clone(),trader.clone(), "Closed Trade", false).await{
                        Ok(_val) => (),
                        Err(err)=>{
                            println!("{}", err);
                        }
                    };
                    sleep(Duration::from_secs(configs.delai)).await;
                }
                history.clear();
                history = save;
            }
            tmp.clear();
        }
        sleep(Duration::from_secs(3)).await;
    }
}
