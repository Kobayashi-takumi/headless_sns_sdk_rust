pub mod client;
pub mod config;
pub mod error;
pub mod post;
pub mod prelude;

#[async_trait::async_trait]
trait Executor<T> {
    async fn execute(&self, client: &client::Client) -> Result<T, Box<dyn std::error::Error>>;
}
