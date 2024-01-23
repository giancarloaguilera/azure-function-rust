use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: i64,
    first_name: String,
    last_name: String,
    email: String,
    department: String,
    city: String,
    state: String,
    zip: String,
    uuid: String
}

#[tokio::main]
async fn main() {
    let host = warp::get()
        .and(warp::path("api"))
        .and(warp::path("HttpExample"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|query: HashMap<String, String>|
            Response::builder()
                .header("Content-Type", "application/json; charset=UTF-8")
                .body(default_response(&query)));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    println!("Starting the application on port {}...", port);

    warp::serve(host).run((Ipv4Addr::LOCALHOST, port)).await
}

fn populate_users(take: i32) -> Result<Vec<User>, csv::Error>  {
    let mut reader = csv::Reader::from_path("data/system_users.csv")?;
    let mut users = Vec::new();

    let mut rec_num = 0;

    for record in reader.records() {
        if rec_num >= take {
            break;
        }

        let record = record?;

        let user = User {
            id: (&record[0]).parse().unwrap(),
            first_name: (&record[1]).to_string(),
            last_name: (&record[2]).to_string(),
            email: (&record[3]).to_string(),
            department:(&record[4]).to_string(),
            city: (&record[5]).to_string(),
            state: (&record[6]).to_string(),
            zip: (&record[7]).to_string(),
            uuid: (&record[8]).to_string(),
        };

        users.push(user);
        rec_num += 1;
    }

    Ok(users)
}

fn default_response(query: &HashMap<String, String>) -> String {
    let mut take = match query.get("take") {
        Some(take_str) => take_str.parse().unwrap(),
        None => 0,
    };

    if take == 0 {
        take = 10;
    }

    let json;

    match populate_users(take) {
        Ok(users) => {
            json = match serde_json::to_string(&users) {
                Ok(val) => val,
                Err(_) => panic!("JSON error"),
            }
        },
        Err(_) => panic!("Fatal error"),
    }

    json
}
