use std::borrow::Cow;

use anyhow::Result;
use kitty_image::{Action, ActionPut, ActionTransmission, Command};
use reqwest::{IntoUrl, Url, blocking};

struct DevClient {
    inner: blocking::Client,
    base_url: Option<Url>,
}

impl DevClient {
    fn new() -> Self {
        Self {
            inner: blocking::Client::new(),
            base_url: None,
        }
    }

    fn base_url(mut self, into_url: impl IntoUrl) -> Result<Self> {
        self.base_url = Some(into_url.into_url()?);
        Ok(self)
    }

    fn get(&self, partial_url: &str) -> Result<blocking::Response> {
        let url = Url::options()
            .base_url(self.base_url.as_ref())
            .parse(partial_url)?;
        Ok(self.inner.get(url).send()?)
    }
}

fn main() -> Result<()> {
    let client = DevClient::new().base_url("http://localhost:8080")?;

    while !client
        .get("/ready")
        .map(|res| res.status().is_success())
        .unwrap_or(false)
    {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let badge_response = client.get("/deployment/moa-dev/myqueue-contacts/badge")?;
    let badge_bytes = badge_response.bytes()?.into_iter().collect::<Vec<_>>();

    let path = format!("{}/../../target/badge.png", std::env!("CARGO_MANIFEST_DIR"));
    std::fs::write(path, &badge_bytes)?;

    let mut cmd = Command::new(Action::TransmitAndDisplay(
        ActionTransmission {
            format: kitty_image::Format::Png,
            medium: kitty_image::Medium::Direct,
            ..Default::default()
        },
        ActionPut {
            move_cursor: true,
            ..Default::default()
        },
    ));
    cmd.payload = Cow::Borrowed(&badge_bytes);
    let cmd = kitty_image::WrappedCommand::new(cmd);
    println!("{cmd}");

    Ok(())
}
