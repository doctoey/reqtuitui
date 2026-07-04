use std::time::Instant;

use reqwest::{Client, Method};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://jsonplaceholder.typicode.com/todos/1";

    println!("Sending Get request to {}...", url);
    let start_time = Instant::now();

    // Fire the request
    let response = client
        .request(Method::GET, url)
        .header("User-Agent", "RequestTui/1.0")
        .send()
        .await?;

    let duration = start_time.elapsed();
    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;

    // Output the results
    println!("\n--- Status: {} ({}ms) ---", status, duration.as_millis());

    println!("\n--- Headers ---");
    for (key, value) in headers.iter() {
        println!("{}: {:?}", key, value);
    }

    println!("\n--- Body ---");
    println!("{}", body);

    Ok(())
}
