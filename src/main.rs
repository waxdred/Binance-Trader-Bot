#[cfg(debug_assertions)]
mod post;
mod config;
mod models;
mod binance;
mod webhook;
mod proxy;
use models::InfoUids;

async fn get_infos(url:Vec<String>)->Vec<InfoUids>{
    let mut infos :Vec<InfoUids> = Vec::new();
    for u in url.iter(){
        match post::post_requet(u.to_string()).await{
            Ok(info) => {
                // println!("{:#?}", info);
                infos.insert(0, InfoUids::new(info, u));
            },
            Err(err)=>{
                println!("{}", err);
            }
        };
    }
    infos
}

#[tokio::main]
async fn main(){
    let mut configs = match config::config(){
        Ok(configs)=> configs,
        Err(_error)=>{
            println!("{:?}", _error);
            return;
        }
    };
    proxy::Proxy::new().await;
    let mut tasks = Vec::new();
    configs.get_uid();
    let infos = get_infos(configs.url.clone()).await;
    for uid in infos{
        let config = configs.clone();
        let task = tokio::spawn(async move{
            println!("new task");
            match binance::follow_trade(uid.uid.clone(), config).await{
                Ok(_val) => (),
                Err(err) =>{
                    println!("{}", err);
                }

            };
        });
        tasks.push(task)
    }
    for t in tasks{
        t.await.unwrap();
    }
}
