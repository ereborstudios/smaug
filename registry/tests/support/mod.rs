use std::net::TcpListener;
use std::result::Result;

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        let server = smaug_registry::app(listener).expect("Failed to bind address");
        let _ = tokio::spawn(server);

        TestApp { address }
    }

    pub async fn get<S: AsRef<str>>(
        &self,
        location: S,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        client
            .get(&format!("{}{}", self.address, location.as_ref()))
            .send()
            .await
    }
}
