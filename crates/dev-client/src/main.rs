use anyhow::Result;
use kitty_image::{Action, ActionPut, ActionTransmission, Command};
use reqwest::{IntoUrl, Method, StatusCode, Url, blocking, header::HeaderMap};

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

    fn get(&self, partial_url: &str) -> Result<DevResponse> {
        let url = Url::options()
            .base_url(self.base_url.as_ref())
            .parse(partial_url)?;

        let response = self.inner.get(url.clone()).send()?;

        let status = response.status();

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok());
        let content_length = response.content_length();

        let headers = response.headers().clone();

        let body = match (content_type, content_length) {
            (Some("image/png"), _) => Body::Png(response.bytes()?.to_vec()),
            (_, Some(0)) => Body::Empty,
            (_, _) => Body::Text(response.text()?),
        };

        Ok(DevResponse {
            request_method: Method::GET,
            request_url: url,
            status,
            headers,
            body,
        })
    }
}

struct DevResponse {
    request_method: Method,
    request_url: Url,
    status: StatusCode,
    headers: HeaderMap,
    body: Body,
}

impl DevResponse {
    /// Formatting taken from the [httpc-test] crate.
    ///
    /// [httpc-test]: https://github.com/jeremychone/rust-httpc-test
    fn print(&mut self) {
        println!();
        println!("=== {} {}", self.request_method, &self.request_url);

        println!(
            " => {:<15}: {} {}",
            "Status",
            self.status.as_str(),
            self.status.canonical_reason().unwrap_or_default()
        );

        println!(" => {:<15}:", "Headers");
        for (n, v) in self.headers.iter() {
            println!("      {}: {}", n, v.to_str().unwrap_or_default());
        }

        if !self.body.is_empty() {
            println!(" => {:<15}:", "Response Body");
            // println!("{:?}", self.body.as_ref());
            match &self.body {
                Body::Png(bytes) => print_png(bytes),
                Body::Text(text) => println!("{text}"),
                Body::Empty => unreachable!(),
            }
        }

        println!("===");
    }
}

enum Body {
    Text(String),
    Png(Vec<u8>),
    Empty,
}

impl Body {
    fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

fn print_png(bytes: &[u8]) {
    // let path = format!("{}/../../target/badge.png", std::env!("CARGO_MANIFEST_DIR"));
    // std::fs::write(path, bytes)?;

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
    cmd.payload = bytes.into();
    let cmd = kitty_image::WrappedCommand::new(cmd);
    println!("{cmd}");
}

fn main() -> Result<()> {
    let client = DevClient::new().base_url("http://localhost:8080")?;

    while !client
        .get("/ready")
        .map(|res| res.status.is_success())
        .unwrap_or(false)
    {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    client
        .get("/deployment/moa-dev/myqueue-contacts/containers/badge")?
        .print();

    client
        .get("/deployment/moa-dev/myqueue-contacts/version/badge")?
        .print();

    Ok(())
}
