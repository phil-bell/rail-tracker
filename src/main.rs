#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

.io/api/v1/json/search/LMS")


556070bcdea17cf74d0d8f15f"),



alue>()


    let res = client
        .get(
            "https://api.rtt.io/api/v1/json/service/C79960/2024/08/17
",
        )
        .basic_auth(
            "",
            Some(""),
        )
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{res:#?}");

    Ok(())
}
