use reqwest::Error;
use serde::Deserialize;
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Response {
    url: String,
    args: serde_json::Value,
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let data = fetch_data(5);
    let time_since = calculate_last_login();
    let (posts, ()) = tokio::join!(data, time_since);
    let duration = start_time.elapsed();

    println!("Posts are: {posts:?}");
    println!("Duration of the program: {duration:?}");
}

async fn fetch_data(seconds: u64) -> Result<Response, Error> {
    let request_url = format!("https://httpbin.org/delay/{seconds}");
    let response = reqwest::get(&request_url).await?;
    let delayed_response_data: Response = response.json().await?;
    Ok(delayed_response_data)
}

async fn calculate_last_login() {
    sleep(Duration::from_secs(1)).await;
    println!("Last login was 2 days ago");
}
