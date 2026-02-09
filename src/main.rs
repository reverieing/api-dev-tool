use api_dev_tool::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::default_config()?;

    let response = client.get("https://jsonplaceholder.typicode.com/posts/1").await?;
    
    println!("{}", response);

    Ok(())
}