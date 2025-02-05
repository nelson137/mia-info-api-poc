use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    while !client
        .do_get("/ready")
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
    {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    client.do_get("/ready").await?.print().await?;

    client
        .do_get("/deployment/moa-dev/myqueue-contacts/badge")
        .await?
        .print()
        .await?;

    Ok(())
}
