use reqwest;
use serde_json::Value;

#[tokio::main]
async fn fetch_ip_info() -> Result<Value, Box<dyn std::error::Error>> {
    println!("🌐 Fetching your IP information from ipinfo.io...\n");

    let client = reqwest::Client::new();
    let response = client.get("https://ipinfo.io/json").send().await?;
    
    if !response.status().is_success() {
        println!("Error: Failed to fetch IP information (status: {})", 
                 response.status());
        return Err("Failed to fetch".into());
    }

    let json: Value = response.json().await?;
    Ok(json)
}

#[tokio::main]
async fn display_ip_info(json: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let ip = json["ip"].as_str().unwrap_or("N/A");
    let city = json["city"].as_str().unwrap_or("N/A");
    let region = json["region"].as_str().unwrap_or("N/A");
    let country = json["country"].as_str().unwrap_or("N/A");
    
    println!("✅ Your IP Information:");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📍 IP Address: {}", ip);
    println!("🏢 City:       {}", city);
    println!("🌍 Region:     {}", region);
    println!("🏴 Country:    {}", country);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    Ok(())
}