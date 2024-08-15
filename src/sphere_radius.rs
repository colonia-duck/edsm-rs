pub mod sphere_radius_request {
    use maplit::hashmap;
    use serde::{Deserialize, Serialize};
    use std::io;

    #[derive(Debug, Serialize, Deserialize)]
    struct SphereInfo {
        name: String,
        #[serde(rename = "bodyCount")]
        body_count: u32,
        distance: f32,
        id: u64,
        coords: Coords,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Coords {
        x: f64,
        y: f64,
        z: f64,
    }

    pub async fn system_name_sphere() -> Result<(), reqwest::Error> {
        println!("please enter the name of the system you want the sphere to be centered on.");

        let mut name: String = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        let input_name = name.trim();

        println!("please enter radius of the sphere (from 1-100).");

        let mut radius: String = String::new();
        io::stdin()
            .read_line(&mut radius)
            .expect("Failed to read line");

        let input_radius = radius.trim();

        sphere_radius(input_name, input_radius).await?;

        Ok(())
    }

    pub async fn sphere_radius(
        system_name: &str,
        sphere_radius: &str,
    ) -> Result<(), reqwest::Error> {
        let url = "https://www.edsm.net/api-v1/sphere-systems";

        let params = hashmap! {
            "systemName" => system_name,
            "radius" => sphere_radius,
            "showId" => "1",
            "showCoordinates" => "1"
        };

        let response: serde_json::Value = reqwest::Client::new()
            .get(url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        let sphere_output: Vec<SphereInfo> =
            serde_json::from_value(response).expect("Failed to deserialize JSON");
        println!("\n{:#?}", sphere_output);
        Ok(())
    }
}
