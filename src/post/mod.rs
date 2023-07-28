mod model;

use super::Executor;
use crate::client::Client;
pub use model::*;
use serde_json::from_str;

pub struct Post {
    path: String,
}

impl Post {
    pub fn new() -> Self {
        Self {
            path: "/posts".to_string(),
        }
    }
    pub fn get(&self, id: String) -> Get {
        Get {
            id,
            path: self.path.clone(),
        }
    }
    pub fn list(&self) -> List {
        List {
            path: self.path.clone(),
        }
    }
    pub fn create(&self, request: CreateRequest) -> Create {
        Create {
            request,
            path: self.path.clone(),
        }
    }
}

pub struct Get {
    id: String,
    path: String,
}

#[async_trait::async_trait]
impl Executor<PostResponse> for Get {
    async fn execute(&self, client: &Client) -> Result<PostResponse, Box<dyn std::error::Error>> {
        let res = client
            .get(format!("{}/{}", self.path, self.id).as_str())
            .await?;
        let res: PostResponse = from_str(res.text().await?.as_str())?;
        Ok(res)
    }
}

pub struct List {
    path: String,
}

#[async_trait::async_trait]
impl Executor<Vec<PostResponse>> for List {
    async fn execute(
        &self,
        client: &Client,
    ) -> Result<Vec<PostResponse>, Box<dyn std::error::Error>> {
        let res = client.get(&self.path).await?;
        let res: Posts = from_str(res.text().await?.as_str())?;
        Ok(res.items)
    }
}

pub struct Create {
    request: CreateRequest,
    path: String,
}

#[async_trait::async_trait]
impl Executor<()> for Create {
    async fn execute(&self, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let res = client.post(&self.path, self.request.clone()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::config::ConfigBuilder;

    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn create() -> Result<()> {
        let client = Client::new(ConfigBuilder::new().api_key("").build()?).unwrap();
        let res = Post::new()
            .create(
                CreateRequest::builder()
                    .content("test".to_string())
                    .build()?,
            )
            .execute(&client)
            .await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn list_and_then_get() -> Result<()> {
        let client = Client::new(ConfigBuilder::new().api_key("").build()?).unwrap();
        let res = Post::new().list().execute(&client).await;
        assert!(res.is_ok());
        let target = res.unwrap()[0].id.clone();
        let res = Post::new().get(target).execute(&client).await;
        assert!(res.is_ok());
        Ok(())
    }
}
