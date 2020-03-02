use async_trait::async_trait;
use reqwest::{Error};

#[async_trait]
pub trait Client {
    async fn get_subpage_html_body(&self, _subpage_uri: &str) -> Result<String, Error>;
}
