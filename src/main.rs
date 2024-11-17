#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY")?;
    let base_url = "https://www.ncei.noaa.gov/cdo-web/api/v2/locations";
    let client = reqwest::Client::new();
    let res = client.get(base_url).header("token", api_key).send().await?;

    println!("res = {res:?}");
    Ok(())
}
