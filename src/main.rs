use reqwest::Error; // Import Error type from reqwest

async fn fetch_data() -> Result<(), Error> {
    let url = "https://www.edsm.net/api-v1/system";

    let mut params = std::collections::HashMap::new();
    params.insert("systemName", "Di yomi");
    params.insert("showInformation","1");
    params.insert("showPrimaryStar","1");
    
   

    let client = reqwest::Client::new();
    let response = client.get(url)
        .query(&params)
        .send()
        .await?;


    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read the response body as a string
        let body = response.text().await?;
        println!("Response body: {}", body);
    } else {
        println!("Error: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    fetch_data().await?;
    Ok(())
}