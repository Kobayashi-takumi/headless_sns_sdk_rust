use crate::error::BuilderError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub api_key: String,
    pub api_url: String,
}

impl Config {
    pub fn new(api_key: &str, api_path: &Option<String>) -> Self {
        Self {
            api_key: api_key.to_string(),
            api_url: api_path
                .clone()
                .unwrap_or("http://localhost:8080".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConfigBuilder {
    api_key: Option<String>,
    api_url: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            api_key: Default::default(),
            api_url: Default::default(),
        }
    }
    pub fn api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }
    pub fn api_url(mut self, api_url: &str) -> Self {
        self.api_url = Some(api_url.to_string());
        self
    }
    pub fn build(&self) -> Result<Config, BuilderError> {
        let api_key = match &self.api_key {
            Some(val) => val,
            _ => return Err(BuilderError::Required("api_key".to_string())),
        };
        Ok(Config::new(&api_key, &self.api_url))
    }
}
