use rand::seq::SliceRandom;
#[derive(Debug, Clone)]
pub struct Proxy{
    pub proxy: Vec<Conn>,
}

#[derive(Debug, Clone)]
pub struct Conn{
    pub ip: String,
    pub port: String,
}

impl Proxy{
    pub async fn new()->Proxy{
        let url = "https://raw.githubusercontent.com/clarketm/proxy-list/master/proxy-list-raw.txt";
        let response = reqwest::get(url).await.unwrap();
        let listing = response.text().await.unwrap();
        let line:Vec<&str> = listing.split('\n').collect();
        let mut list:Vec<Conn> = Vec::new(); 
        for l in line.iter(){
            let line = l.to_string();
            let sp:Vec<&str> = line.split(':').collect();
            if sp.len() > 1{
                list.insert(0, Conn{
                    ip: sp[0].to_string(),
                    port: sp[1].to_string(),
                })
            }
        }
       Proxy{
           proxy: list
       } 
    }
    pub fn random(&self)->&Conn{
        let ret = self.proxy.choose(&mut rand::thread_rng()).unwrap();
        ret
    }
}
