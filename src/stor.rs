use actix_web::{Responder, HttpRequest, HttpResponse, post, web};
use actix_http::body::to_bytes;
use uuid::Uuid;
use chrono::prelude::*;
use redis::Commands;
use std::env;

#[allow(unused)]
#[post("/api/storage")]
pub async fn gon(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let peer = req.peer_addr();
    let requ = req.headers();
    let readi: DateTime<Utc> = Utc::now();
    log::info!("{} - {:?} - /api/storage POST request (fetch redis key) - from {:?} - {:?}", readi, &txid, peer, &requ);
    let bbod = to_bytes(body).await.unwrap();
    let sbod = std::str::from_utf8(&bbod).unwrap();
    let gotit = format!("{{ \"data\": {:?} }}", redisget(sbod).await.unwrap()); 
    let nid = env::var("txid").unwrap();
    let reada: DateTime<Utc> = Utc::now();
    log::info!("{} - {} - /api/storage response from redis - {:?}", reada, &nid, &gotit);
    HttpResponse::Ok().body(gotit)
}

#[allow(unused)]
pub async fn redisget(getit: &str) -> redis::RedisResult<String> {
    let redis_client = redis::Client::open("redis://localhost:6379/")?;
    let mut rcon = redis_client.get_connection()?;
    let some_value: Result<std::string::String, _> = match rcon.get(getit) {
        Ok(redis::Value::Nil) => {
            let fu: String = "Not found.".to_string();
            Err(fu)
        },
        Ok(redis::Value::Data(bytes)) => {
            let val: &str = std::str::from_utf8(&bytes).unwrap();
            let fu: String = val.to_string();
            Ok(fu)
        },
        _ => Ok("Not found".to_string()),
    };

   let mut return_me = String::new();

    match some_value {
        Err(_) => return_me = "Not found.".to_string(), 
        _ => return_me = some_value.unwrap(),
    }

    Ok(return_me)
}
