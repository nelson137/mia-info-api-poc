use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = httpc_test::new_client("http://localhost:8080")?;

    client
        .do_get("/hello?name=Prometheus")
        .await?
        .print()
        .await?;

    client.do_get("/hello2/Prometheus2").await?.print().await?;

    Ok(())
}
