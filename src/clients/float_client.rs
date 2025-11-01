use reqwest;
/// referencing https://www.petergirnus.com/blog/how-to-make-http-requests-in-rust

pub struct FloatClient {
    id: String,
}

impl FloatClient {
    pub fn new() -> Self {
        FloatClient {
            id: String::from("randomid_123"),
        }
    }

    #[tokio::main]
    pub async fn authenticate(&self) {
        let resp = match reqwest::get("https://dog.ceo/api/breeds/image/random").await {
            Ok(resp) => resp.text().await.unwrap(),
            Err(err) => panic!("Error: {}", err),
        };
        println!("{}", resp)
    }
}
