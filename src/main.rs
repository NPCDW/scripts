#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::app_config::get_config();

    config::log::init(&config);
    tracing::debug!("Read Config: {:#?}", &config);

    anyhow::Ok(())
}
