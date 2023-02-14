#[cfg(debug_assertions)]
use std::time::Duration;
use tokio::time::sleep;

use crate::{post, webhook, models::{self, OtherPositionRetListBool}};

pub async fn follow_trade(uid: String, configs: models::Config)->Result<(), reqwest::Error>{
    let mut history:Vec<models::OtherPositionRetListBool> = Vec::new();
    let trader = match post::post_requet(uid.clone()).await{
        Ok(trader)=>trader,
        Err(err)=> {
            println!("{:#?}", err);
            return Err(err);
        }
    };
    loop {
        // add try catch
        println!("Start new trade");
        let trade = match post::post_get_trade(uid.clone()).await{
            Ok(trade)=> trade,
            Err(_err)=> {
                continue;
            }
        };
        println!("trade :{:#?}", trade);
        if !trade.data.other_position_ret_list.is_empty(){
            let mut tmp = trade.data.other_position_ret_list;
            if tmp.len() > history.len(){
                for t in tmp.iter(){
                    let mut check = true;
                    for h in history.iter(){
                        if t.update_time_stamp == h.data.update_time_stamp{
                            check = false;
                        }
                    }
                    if check{
                        //send to webhook new trade
                        if !configs.whitelist.is_empty() {
                            if configs.whitelist.iter().any(|x| x == &t.symbol.clone()){
                                let data = OtherPositionRetListBool::new(t.clone(), true);
                                if t.pnl > -0.10 && t.pnl < 0.10{
                                    history.insert(0, data.clone());
                                }else{
                                    data.clone().change_trade(false);
                                    history.insert(0, data.clone());
                                }
                                if data.trade{
                                    match webhook::send_webhook(t.clone(), configs.clone(),trader.clone(), "New Trade", true).await{
                                       Ok(_val) => (),
                                       Err(err)=>{
                                           println!("{}", err);
                                       }
                                    };
                                }
                            }
                        }else if !configs.blacklist.is_empty() && configs.blacklist.contains(&t.symbol.clone()){
                            continue;
                        }else{
                            let data = OtherPositionRetListBool::new(t.clone(), true);
                            if t.pnl > -0.10 && t.pnl < 0.10{
                                history.insert(0, data.clone());
                            }else{
                                data.clone().change_trade(false);
                                history.insert(0, data.clone());
                            }
                            if data.trade{
                                match webhook::send_webhook(t.clone(), configs.clone(),trader.clone(), "New Trade", true).await{
                                    Ok(_val) => (),
                                    Err(err)=>{
                                        println!("{}", err);
                                    }
                                };
                            }
                        }
                        sleep(Duration::from_secs(configs.delai)).await;
                        //add time sleep
                    }
                }
            } 
            if !history.is_empty(){
                let save = history.clone();
                // println!("tmp {:#?}", tmp.clone());
                history.retain(|x| !tmp.iter()
                               .any(|y| y.update_time_stamp == x.data.update_time_stamp && y.symbol == x.data.symbol));
                // println!("history {:#?}", history.clone());
                for h in history.iter(){
                    //close trade 
                    if h.trade{
                        match webhook::send_webhook(h.data.clone(), configs.clone(),trader.clone(), "Closed Trade", false).await{
                            Ok(_val) => (),
                            Err(err)=>{
                                println!("{}", err);
                            }
                        };
                        sleep(Duration::from_secs(configs.delai)).await;
                    }
                }
                history.clear();
                history = save;
            }
            tmp.clear();
        }
        sleep(Duration::from_secs(3)).await;
    }
}
