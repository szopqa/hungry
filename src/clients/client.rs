use async_trait::async_trait;
use reqwest::{Error};

#[async_trait]
pub trait Client {
//    fn new(_base_uri: &str) -> Client;
    async fn get_subpage_html_body(&self, _subpage_uri: &str) -> Result<String, Error>;
}
