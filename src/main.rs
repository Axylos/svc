use reqwest;
use std::collections::HashMap;
use serde::Deserialize;
use serde_json;

fn fetch() -> Result<(Vec<Person>), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://swapi.co/api/people/")?
        .json::<HashMap<String, serde_json::Value>>()?;

    match resp.get("results") {
        Some(resp) => {
            let people: &Vec<Person> = resp.as_array().unwrap();
            //let data: Vec<Person> = resp;
            Ok(people.to_vec())

        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Person {
    gender: String,
    name: String,

}

fn print_data(data: Result<Vec<Person>, Box<dyn std::error::Error>>) -> Option<u64>{
    match data {
        Ok(data) => {
            println!("{:?}", data);
            Some(4)

        }
        Err(e) => None
    }

}
fn main() {
    let data = fetch();
    print_data(data);
}
