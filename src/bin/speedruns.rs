#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    speedruns_cli::main().await
}
