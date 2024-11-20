mod config;
mod service;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::app_config::get_config();

    config::log::init(&config);
    tracing::debug!("Read Config: {:#?}", &config);

    service::akile_checkin_svc::checkin_akile(&config).await;

    anyhow::Ok(())
}
