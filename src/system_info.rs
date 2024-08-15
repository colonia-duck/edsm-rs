pub mod system_information_request {

    use maplit::hashmap;
    use serde::{Deserialize, Serialize};
    use std::io;

    #[derive(Debug, Serialize, Deserialize)]
    struct SystemInfo {
        name: String,
        coords: Coords,
        information: Information,
        #[serde(rename = "primaryStar")]
        primary_star: PrimaryStar,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Coords {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Information {
        allegiance: String,
        economy: String,
        faction: String,
        #[serde(rename = "factionState")]
        faction_state: String,
        government: String,
        population: u64,
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

    pub async fn system_name() -> Result<(), reqwest::Error> {
        println!("please enter the name of the system you want");

        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let param_value = input.trim();

        system_information(param_value).await?;

        Ok(())
    }

    pub async fn system_information(param_value: &str) -> Result<(), reqwest::Error> {
        let url = "https://www.edsm.net/api-v1/system";

        let params = hashmap! { // values to be sent to the api to show certain information.
            "systemName" => param_value,
            "showInformation" => "1",
            "showPrimaryStar" => "1",
            "showCoordinates" => "1"
        };

        let response: serde_json::Value = reqwest::Client::new()
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        let system_output: SystemInfo =
            serde_json::from_value(response).expect("Failed to deserialize JSON");
        println!("{:#?}", system_output);

        Ok(())
    }
}
