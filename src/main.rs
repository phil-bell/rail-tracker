#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.rtt.io/api/v1/json/service/C79960/2024/08/17")
        .basic_auth("", Some(""))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{res:#?}");

    Ok(())
}
