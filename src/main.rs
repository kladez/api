#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kladez_api::run().await
}
