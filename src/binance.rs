use crate::{models::{self, OtherPositionRetList}, post, webhook};

pub async fn follow_trade(uid: String, configs: models::Config){
    let mut history:Vec<models::OtherPositionRetList> = Vec::new();
    loop {
        // add try catch
        let mut error = false;
        let trade = match post::post_get_trade(uid.clone()).await{
            Ok(trade)=> trade,
            Err(err)=> {
                println!("{:#?}", err);
                error = true;
                continue;
            }
        };
        if !error && !trade.data.other_position_ret_list.is_empty(){
            let tmp = trade.data.other_position_ret_list;
            if tmp.len() > history.len(){
                for t in tmp.iter(){
                    let mut check = true;
                    for h in history.iter(){
                        if t == h{
                            check = false;
                        }
                    }
                    if check{
                        //send to webhook new trade
                        history.insert(0, t.clone());
                        webhook::send_webhook(t.clone(), configs.clone()).await;
                        //add time sleep
                    }
                }
            } 
            for h in history.iter_mut(){
                let mut check = false;
                let mut tmpTrade: &OtherPositionRetList;
                for t in tmp.iter(){
                    if h == t {
                        check = true;
                        tmpTrade = h;
                    }
                }
                if !check{
                    history.retain(|x| x == tmpTrade);
                }
            }
        }
    }
}
