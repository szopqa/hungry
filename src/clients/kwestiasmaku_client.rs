use async_trait::async_trait;
use reqwest::{Error, get, Url};

use super::client::{Client};

#[derive(Debug)]
pub struct KwestiasmakuClient {
    _base_uri: reqwest::Url,
    _http_client: reqwest::Client
}

#[async_trait]
impl Client for KwestiasmakuClient {
    fn new(_base_uri: &str) -> KwestiasmakuClient {
        KwestiasmakuClient {
            _base_uri: Url::parse(_base_uri).unwrap(),
            _http_client: reqwest::Client::new()
        }
    }

    fn get_base_uri (&self) -> &Url {
        &self._base_uri
    }

    async fn get_subpage_html_body(&self, _subpage_uri: &str) -> Result<String, Error> {
        let _request_uri = self._base_uri.join(_subpage_uri).unwrap();
        let body = get(_request_uri).await?.text().await?;
        Ok(body)
    }
}
