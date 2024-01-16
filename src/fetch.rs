use actix_web::{Responder, HttpRequest, HttpResponse, post, web};
use actix_http::body::to_bytes;
use uuid::Uuid;
use chrono::prelude::*;
use redis::Commands;
use std::env;

#[allow(unused)]
#[post("/api/fetch")]
pub async fn rtch(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    log::info!("{} - {} - /api/fetch POST->GET request (fetch body hash) - from {:?} - {:?}", readi, &txid, peer, &requ);
    let mut bbod = to_bytes(body).await.unwrap();
    let sbod: Result<String, _> = match std::str::from_utf8(&bbod) {
        Ok(string) => {
            let seasnails = std::str::from_utf8(&bbod).unwrap().to_string();    
            Ok(seasnails)
        },
        _ => {
            let seasnails = "ERROR: non-utf8 data received.".to_string();
            Err(seasnails)
        }
    };
    
    let mut return_me = String::new();
    match sbod {
        Err(_) => return_me = "ERROR: non-utf8 data received.".to_string(), 
        _ => return_me = sbod.unwrap(),
    }

    let rbod = return_me;
    let nid = env::var("txid").unwrap();
    let jelly = ifetch(&rbod).await;
    let mut returnbod = String::new();
    match &jelly {
        Ok(()) => returnbod = format!("{{ \"{:?}\": \"{}\" }}", jelly, nid),
        _ => returnbod = format!("{{ \"ERROR\": \"{}\" }}", nid)
    };
    let readz: DateTime<Utc> = Utc::now();
    log::info!("{} - {} - fetch debug data: {:?}", readz, &txid, jelly);
    let reada: DateTime<Utc> = Utc::now();
    log::info!("{} - {} - /api/fetch response from callback recv - {:?}", reada, &nid, requ);
    HttpResponse::Ok().body(returnbod)
}

pub async fn ifetch(url: &str) -> Result<(), reqwest::Error> {
    let nid = env::var("txid").unwrap();
    let timed = Utc::now();
    log::info!("{} - {} - Fetching {:?}...", timed, &nid, url);
    let res = reqwest::get(url).await?;
    let timeo = Utc::now();
    log::info!("{} - {} - Response: {:?} {}", &timeo, &nid, res.version(), res.status());
    let body = res.text().await?;
    let timeb = Utc::now();
    let mut hasher = blake3::Hasher::new();
    hasher.update(body.as_bytes());
    let blake3 = hasher.finalize();
    let hashstring = format!("{}", blake3);
    let _belly = ron(url.to_string(), hashstring).await;
    log::info!("{} - {} - {}", timeb, &nid, blake3);
    Ok(())
}

pub async fn ron(url: String, hash: String) {
    let nid = env::var("txid").unwrap();
    let reada: DateTime<Utc> = Utc::now();
    log::info!("{} - {} - {} callsoup tx recv insert HASH: {}", &reada, nid, &url, &hash);
    let _insertres = redisset(url, hash); 
}

pub fn redisset(insertit: String, valit: String) -> redis::RedisResult<()> {
    let redis_client = redis::Client::open("redis://localhost:6379/")?;
    let mut rcon = redis_client.get_connection()?;
    let _seasnails: String = rcon.set(insertit, valit).unwrap();
    Ok(())
}
