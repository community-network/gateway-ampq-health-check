use reqwest::header::HeaderMap;
use serde_json::json;
mod structs;
use std::collections::HashMap;
use std::env;
use std::process::exit;

fn get_server_id() -> structs::CurrentServer {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    match client
        .get("https://api.gametools.network/bfv/detailedserver/?name=%20&platform=pc&lang=en-us")
        .send()
    {
        Ok(response) => match response.json::<structs::CurrentServer>() {
            Ok(current_server) => current_server,
            Err(e) => {
                println!("Couldn't a gameid of a server! - {}", e);
                exit(1)
            }
        },
        Err(e) => {
            println!("Cant access gametools! - {}", e);
            exit(1)
        }
    }
}

fn get_players(gt_gateway: i64, game_id: String) {
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "authentication",
        ((chrono::Utc::now().timestamp() / 60) * gt_gateway).to_string()[..]
            .parse()
            .unwrap(),
    );
    match client
        .post("https://localhost:3030/casablanca/players")
        .headers(headers)
        .json(&json!({
            "game_ids": [game_id.parse::<i64>().unwrap()],
        }))
        .send()
    {
        Ok(response) => match response.json::<Vec<HashMap<i64, structs::GamePlayers>>>() {
            Ok(result) => {
                if result
                    .iter()
                    .any(|i| i.contains_key(&game_id.parse::<i64>().unwrap()))
                {
                    exit(0)
                }
                exit(1)
            }
            Err(e) => {
                println!("Wrong info return! - {}", e);
                exit(1)
            }
        },
        Err(e) => {
            println!("Couldn't get local! - {}", e);
            exit(1)
        }
    }
}

fn main() {
    match dotenvy::dotenv() {
        Ok(_) => {}
        Err(_) => println!(".env not found, using env variables..."),
    };

    let gt_gateway: i64 = env::var("GT_GATEWAY")
        .expect("GT_GATEWAY wasn't set")
        .parse()
        .expect("GT_GATEWAY should be a number");
    let server_info = get_server_id();
    get_players(gt_gateway, server_info.game_id);
}
