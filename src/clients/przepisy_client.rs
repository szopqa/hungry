use async_trait::async_trait;
use reqwest::{Error, get, Url};

use super::client::{Client};

#[derive(Debug)]
pub struct PrzepisyClient {
    _base_uri: reqwest::Url,
    _http_client: reqwest::Client
}

/* Implementing Client trait only for reference type as we don't want to duplicate clients for same host */
#[async_trait]
impl <'a> Client for &'a PrzepisyClient {
    fn get_base_uri (&self) -> &Url {
        &self._base_uri
    }

    async fn get_subpage_html_body(&self, _subpage_uri: &str) -> Result<String, Error> {
        let _request_uri = self._base_uri.join(_subpage_uri).unwrap();
        let body = get(_request_uri).await?.text().await?;
        Ok(body)
    }
}

