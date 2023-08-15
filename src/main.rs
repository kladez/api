#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    prorub_api::run().await
}
