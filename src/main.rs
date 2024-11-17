use reqwest::Url;
//{"mindate":"1876-01-01","maxdate":"2024-11-14","name":"Berlin, GM","datacoverage":1,"id":"CITY:GM000001"}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY")?;
    let base_url = "https://www.ncei.noaa.gov/cdo-web/api/v2/locations";
    let url = Url::parse_with_params(
        base_url,
        &[
            ("locationcategoryid", "CITY"),
            ("sortfield", "name"),
            ("sortorder", "asc"),
            ("limit", "1000"),
        ],
    )?;

    let client = reqwest::Client::new();
    let res = client.get(url).header("token", api_key).send().await?;

    let text = res.text().await?;

    //let json: serde_json::Value = serde_json::from_str(&text)?;
    //let json_pretty = serde_json::to_string_pretty(&json)?;
    println!("res = {text}");
    Ok(())
}
