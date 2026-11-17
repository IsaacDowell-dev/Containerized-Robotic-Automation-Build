use rust_ethernet_ip::{EipClient, PlcValue};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
// }

// #[tokio::main]
pub async fn read_plc_tags() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Read plc tags called");

    let mut client = EipClient::connect("192.168.6.32:2000").await?;

    let tags = ["Local:1:I.Data.8", "Local:1:I.Data.9"];
    let results = client.read_tags_batch(&tags).await?;

    for (tag_name, result) in results {
        match result {
            Ok(value) => println!("{}: Succcess: {:?}", tag_name, value),
            Err(e) => println!("{}: Error: {}", tag_name, e),
        }
    }

    Ok(())
}