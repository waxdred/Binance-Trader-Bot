use std::time::Duration;
use tokio::time::sleep;

use crate::{post, webhook, models};

pub async fn follow_trade(uid: String, configs: models::Config)->Result<(), reqwest::Error>{
    let mut history:Vec<models::OtherPositionRetList> = Vec::new();
    println!("Start new trade");
    let trader = match post::post_requet(uid.clone()).await{
        Ok(trader)=>trader,
        Err(err)=> {
            return Err(err);
        }
    };
    loop {
        // add try catch
        let mut error = false;
        let trade = match post::post_get_trade(uid.clone()).await{
            Ok(trade)=> trade,
            Err(_err)=> {
                error = true;
                continue;
            }
        };
        if !error && !trade.data.other_position_ret_list.is_empty(){
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
                        history.insert(0, t.clone());
                        println!("add trade");
                        match webhook::send_webhook(t.clone(), configs.clone(),trader.clone(), "New Trade").await{
                            Ok(_val) => (),
                            Err(err)=>{
                                println!("{}", err);
                            }
                        };
                        sleep(Duration::from_secs(configs.delai)).await;
                        //add time sleep
                    }
                }
            } 
            if !history.is_empty(){
                history.retain(|x| !tmp.iter().any(|y| y.update_time_stamp == x.update_time_stamp));
                for h in history.iter(){
                    //close trade 
                    println!("Close trade");
                    match webhook::send_webhook(h.clone(), configs.clone(),trader.clone(), "Close Trade").await{
                        Ok(_val) => (),
                        Err(err)=>{
                            println!("{}", err);
                        }
                    };
                    sleep(Duration::from_secs(configs.delai)).await;
                }
                history.clear();
                history = tmp.clone();
            }
            tmp.clear();
        }
        sleep(Duration::from_secs(3)).await;
    }
}
