use crate::error::BuilderError;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: String,
    pub content: String,
    pub hash_tags: Vec<String>,
    pub mentions: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Posts {
    pub items: Vec<PostResponse>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreateRequest {
    content: String,
    hash_tags: Vec<String>,
    mentions: Vec<String>,
}

impl CreateRequest {
    pub fn new(content: String, hash_tags: Vec<String>, mentions: Vec<String>) -> Self {
        Self {
            content,
            hash_tags,
            mentions,
        }
    }
    pub fn builder() -> CreateRequestBuilder {
        CreateRequestBuilder {
            content: Default::default(),
            hash_tags: Default::default(),
            mentions: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct CreateRequestBuilder {
    content: Option<String>,
    hash_tags: Option<Vec<String>>,
    mentions: Option<Vec<String>>,
}

impl CreateRequestBuilder {
    pub fn content(mut self, value: String) -> Self {
        self.content = Some(value);
        self
    }
    pub fn hash_tags(mut self, value: Vec<String>) -> Self {
        self.hash_tags = Some(value);
        self
    }
    pub fn mentions(mut self, value: Vec<String>) -> Self {
        self.mentions = Some(value);
        self
    }
    pub fn build(&self) -> Result<CreateRequest, BuilderError> {
        let content = match &self.content {
            Some(val) => val.clone(),
            _ => return Err(BuilderError::Required("content".to_string())),
        };
        let hash_tags = self.hash_tags.clone().unwrap_or(vec![]);
        let mentions = self.mentions.clone().unwrap_or(vec![]);
        Ok(CreateRequest::new(content, hash_tags, mentions))
    }
}
