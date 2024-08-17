use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = match env::var_os("RTT_USERNAME") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$RTT_USERNAME is not set")
    };
    let secret = match env::var_os("RTT_SECRET") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$RTT_SECRET is not set")
    };
    let client = reqwest::Client::new();
    let res = client
        .get(
            "https://api.rtt.io/api/v1/json/service/C79960/2024/08/17
",
        )
        .basic_auth(username, Some(secret))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{res:#?}");

    Ok(())
}
