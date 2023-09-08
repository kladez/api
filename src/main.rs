#![feature(never_type)]

#[tokio::main]
async fn main() -> Result<!, Box<dyn std::error::Error>> {
    api::run().await
}
