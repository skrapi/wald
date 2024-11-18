use reqwest::Url;
use textplots::{Chart, Plot, Shape};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("API_KEY")?;
    let base_url = "https://www.ncei.noaa.gov/cdo-web/api/v2/data";
    let url = Url::parse_with_params(
        base_url,
        &[
            // Berlin, DE
            ("locationid", "CITY:GM000001"),
            ("limit", "1000"),
            ("datasetid", "GHCND"),
            // Maximum temperature of the day
            ("datatypeid", "TMAX"),
            ("units", "metric"),
            ("startdate", "2020-01-01"),
            ("enddate", "2020-12-31"),
        ],
    )?;

    let client = reqwest::Client::new();
    let res = client.get(url).header("token", api_key).send().await?;

    let text = res.text().await?;

    let json: serde_json::Value = serde_json::from_str(&text)?;
    // {
    //  "attributes": ",,E,",
    //  "datatype": "TMAX",
    //  "date": "2020-01-06T00:00:00",
    //  "station": "GHCND:GME00127438",
    //  "value": 6.6
    //}
    //
    let results = json["results"].clone();

    let temperatures: Vec<(f32, f32)> = results
        .as_array()
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(index, x)| (index as f32, x["value"].as_f64().unwrap() as f32))
        .collect();

    Chart::new(180, 60, 0.0, 1000.0)
        .lineplot(&Shape::Lines(&temperatures))
        .display();

    Ok(())
}
