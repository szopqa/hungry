use async_trait::async_trait;
use reqwest::{Error, Url};

#[async_trait]
pub trait Client {
    fn get_base_uri (&self) -> &Url;

    async fn get_subpage_html_body(&self, _subpage_uri: &str) -> Result<String, Error>;

    fn relative_path_to_full_uri(&self, _relative_path: &str) -> String {
        self.get_base_uri().join(_relative_path).unwrap().as_str().to_string()
    }
}