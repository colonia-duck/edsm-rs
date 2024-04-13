use std::io;

use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize, Deserialize)]
struct EdsmInfo {
    #[serde(rename = "camalCase")]
    name: String,
    information: Vec<Info>,
}
#[derive(Debug,Serialize, Deserialize)]
struct Info {
    allegiance: String,
    government: String,
    faction: String,
    population: i64,
    security: String,
    economy: String,
    #[serde(rename = "secondEconomy")]
    second_economy: String,
    service: String,
    #[serde(rename = "primaryStar")]
    primary_star: String,
    name: String,
    #[serde(rename = "isScoopable")]
    is_scoopable: bool
}
async fn fetch_data(param_value: &str) -> Result<(), reqwest::Error> {
    let url = "https://www.edsm.net/api-v1/system";
    
    let mut params = std::collections::HashMap::new();
    params.insert("systemName", param_value);
    params.insert("showInformation","1");
    params.insert("showPrimaryStar","1");
   

    let response: serde_json::Value = reqwest::Client::new()
        .get(url)
        .query(&params)
        .send()
        .await?
        .json()
        .await?;


    println!("{:#?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    println!("please enter the name of the system you want:");

    let mut input: String = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let param_value = input.trim();

    fetch_data(param_value).await?;
    
    Ok(())
}