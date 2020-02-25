#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    speedruns_cli::main().await
}
