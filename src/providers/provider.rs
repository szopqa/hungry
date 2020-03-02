use async_trait::async_trait;
use reqwest::{Error};

use subpage_config::{SubpageConfig};
use clients::client::{Client};

#[async_trait]
pub trait SubpageDataProvider<T: Client> {
    fn new(_page_config: &SubpageConfig, _page_client: T, _max_iterations_num: i32) -> Self; 
    async fn get_subpage_menu_items(&self, _subpage_uri: &str) -> Result<String, Error>;
}
