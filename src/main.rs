use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    println!("Connecting to: {database_url}");
    Ok(())
}
