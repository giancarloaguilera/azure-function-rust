use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    let example1 = warp::get()
        .and(warp::path("api"))
        .and(warp::path("HttpExample"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("name") {
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });

    let port_key = "3000";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}

//Config tasks
//Figure out how to set the FUNCTIONS_CUSTOMHANDLER_PORT environment variable for the function app.

//Data tasks
//Create CSV file with 10000 user records. See fields below. Mike and Savitha

//Coding tasks
//Create a user type with fields first name, last name, email, phone number, address, state, and zip code.
//Create a function that loads the users JSON file, loops through each record, deserializes each one into a user object, and adds the user object to a list.
//Add new query search parameters first name, last name, email, phone number, address, state, and zip code. 
//For each query parameter filter the results based on the corresponding field.
//For first name use contains logic. For last name use starts with. ???
//Sort results by first name and last name.
//Add paging support to the search.
//Return the results in JSON format.

//Find a API load tester/runner that can be configured to run N number of requests per second for N minutes. Dmitry
