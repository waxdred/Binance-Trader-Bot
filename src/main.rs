
mod post;
mod config;
mod models;
mod binance;
mod webhook;
use models::InfoUid;

async fn get_infos(url:Vec<String>)->Vec<InfoUid>{
    let mut infos :Vec<InfoUid> = Vec::new();
    for u in url.iter(){
        match post::post_requet(u.to_string()).await{
            Ok(info) => infos.push(info),
            Err(_)=>{}
        };
    }
    infos
}

#[tokio::main]
async fn main(){
    let mut configs = match config::config(){
        Ok(configs)=> configs,
        Err(_error)=>{
            println!("config.json not found: {}", _error);
            return;
        }
    };
    let mut tasks = Vec::new();
    configs.get_uid();
    let infos = get_infos(configs.url.clone()).await;
    for uid in configs.url.clone(){
        let config = configs.clone();
        let task = tokio::spawn(async move{
            binance::follow_trade(uid.clone(), config).await;
        });
        tasks.push(task)
    }
    for t in tasks{
        t.await.unwrap();
    }
}
