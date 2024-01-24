use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};
use serde::Serialize;

#[derive(Serialize)]
#[derive(Clone)]
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

// fn main() {
//     let mut query = HashMap::new();

//     query.insert(String::from("first_name"), String::from("M"));

//     let json = default_response(&query);

//     println!("{}", json);
// }

fn populate_users() -> Result<Vec<User>, csv::Error>  {
    let mut reader = csv::Reader::from_path("data/system_users.csv")?;
    let mut users = Vec::new();

    for record in reader.records() {

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

    let users = match populate_users() {
        Ok(usr) => usr,
        Err(_) => Vec::new(),
    };

    let mut filtered_users = filter_users(&users, query);

    filtered_users.sort_by(|a, b| {
        a.first_name.to_lowercase().cmp(&b.first_name.to_lowercase())
            .then(a.last_name.cmp(&b.last_name))
    });

    let json = match serde_json::to_string(&filtered_users) {
        Ok(val) => val,
        Err(_) => panic!("JSON error"),
    };

    json
}

fn filter_users<'a>(users: &'a Vec<User>, query: &HashMap<String, String>) -> Vec<&'a User>  {

    let mut filtered_rec = Vec::new();    

    let first_name = match query.get("firstname") {
        Some(first_name) => first_name.to_lowercase(),
        None => String::from(""),
    };

    let last_name = match query.get("lastname") {
        Some(last_name) => last_name.to_lowercase(),
        None => String::from(""),
    };

    let email = match query.get("email") {
        Some(email) => email.to_ascii_lowercase(),
        None => String::from(""),
    };

    for user in users {
        if (first_name.is_empty() || user.first_name.to_lowercase().starts_with(&first_name))
            && (last_name.is_empty() || user.last_name.to_lowercase().starts_with(&last_name))
            && (email.is_empty() || user.email.to_lowercase().starts_with(&email))
        {
            filtered_rec.push(user);
        }
    }

    filtered_rec
}


