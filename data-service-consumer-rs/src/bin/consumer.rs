use anyhow::{Context, Result};
use app_lib::{config, consumer, db};
use wavesexchange_log::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::load_consumer_config()?;

    info!(
        "Starting data-service consumer with config: {:?}, {:?}",
        config.node, config.consumer
    );

    let conn = db::async_pool(&config.postgres)
        .await
        .context("DB connection failed")?;

    let updates_src = consumer::updates::new(&config.node.blockchain_updates_url)
        .await
        .context("Blockchain connection failed")?;

    let pg_repo = consumer::repo::pg::new(conn);

    let result = consumer::start(
        config.node.starting_height,
        updates_src,
        pg_repo,
        config.node.updates_per_request,
        config.node.max_wait_time,
        config.node.chain_id,
        config.consumer.assets_only,
    )
    .await;

    if let Err(ref err) = result {
        error!("{}", err);
    }
    result
}
