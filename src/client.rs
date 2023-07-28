use crate::config::Config;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client as HttpClient, ClientBuilder, Response,
};
use serde::Serialize;

pub struct Client {
    inner: HttpClient,
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Result<Client, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", config.api_key.as_str()).as_str())?,
        );
        let client = ClientBuilder::new().default_headers(headers).build()?;
        Ok(Self {
            inner: client,
            config,
        })
    }

    pub async fn get(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let res = self.inner.get(&self.path(url)).send().await?;
        Ok(res)
    }

    pub async fn get_with_query<T: Serialize>(
        &self,
        url: &str,
        query: &Option<T>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let mut client = self.inner.get(&self.path(url));
        if let Some(query) = query {
            client = client.query(query);
        }
        let res = client.send().await?;
        Ok(res)
    }

    pub async fn post<T: Serialize>(
        &self,
        url: &str,
        body: T,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let res = self.inner.post(&self.path(url)).json(&body).send().await?;
        Ok(res)
    }

    fn path(&self, url: &str) -> String {
        format!("{}{}", self.config.api_url, url).to_string()
    }
}
