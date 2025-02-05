mod config;
mod service;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::app_config::get_config();

    config::log::init(&config);
    tracing::debug!("Read Config: {:#?}", &config);

    service::aurora_checkin_svc::checkin_aurora(&config).await;

    anyhow::Ok(())
}
