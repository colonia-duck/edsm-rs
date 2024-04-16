use std::io;
use serde::{Deserialize, Serialize};
use maplit::hashmap;

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    information: Information,
    name: String,
    #[serde(rename = "primaryStar")]
    primary_star: PrimaryStar,
}

#[derive(Debug, Serialize, Deserialize)]
struct Information {
    allegiance: String,
    economy: String,
    faction: String,
    #[serde(rename = "factionState")]
    faction_state: String,
    government: String,
    population: i64,
    #[serde(rename = "reserve")]
    reserve: Option<String>,
    #[serde(rename = "secondEconomy")]
    second_economy: String,
    security: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrimaryStar {
    #[serde(rename = "isScoopable")]
    is_scoopable: bool,
    name: String,
}

async fn fetch_data(param_value: &str) -> Result<(), reqwest::Error> {
    let url = "https://www.edsm.net/api-v1/system";
    
    let params = hashmap! {
        "systemName" => param_value,
        "showInformation" => "1",
        "showPrimaryStar" => "1"
    };
    // = std::collections::HashMap::new();
    // params.insert("systemName", param_value);
    // params.insert("showInformation","1");
    // params.insert("showPrimaryStar","1");

    let response: serde_json::Value = reqwest::Client::new()
        .get(url)
        .query(&params)
        .send()
        .await?
        .json()
        .await?;


        let person: SystemInfo = serde_json::from_value(response).expect("Failed to deserialize JSON");
    println!("{:#?}", person);

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
