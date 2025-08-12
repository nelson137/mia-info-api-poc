use axum::{body::Body, http::Response, response::IntoResponse};

pub fn rand_string() -> String {
    rand_string_len(rand::random_range(6..=12))
}

pub fn rand_string_len(len: usize) -> String {
    rand::random_iter::<char>()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .take(len)
        .collect()
}

pub fn rand_vec_u8() -> Vec<u8> {
    rand::random_iter()
        .take(rand::random_range(512..1024))
        .collect()
}

pub trait ReadResponseBody {
    async fn read_response_as_bytes(self) -> Vec<u8>;
    async fn read_response_as_string(self) -> String;
}

impl<T: IntoResponse> ReadResponseBody for T {
    async fn read_response_as_bytes(self) -> Vec<u8> {
        async fn inner(response: Response<Body>) -> Vec<u8> {
            axum::body::to_bytes(response.into_body(), 1024)
                .await
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>()
        }
        inner(self.into_response()).await
    }

    async fn read_response_as_string(self) -> String {
        let bytes = self.read_response_as_bytes().await;
        String::from_utf8_lossy(&bytes).into_owned()
    }
}
