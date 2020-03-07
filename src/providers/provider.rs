use async_trait::async_trait;
use reqwest::{Error};
use std::marker::{Sync, Send};

use crate::clients::client::{Client};
use crate::models::menu::{Menu};
use crate::data_sources::subpage_config::{SubpageConfig};

#[async_trait]
pub trait SubpageDataProvider<T> where T: Client + Sync + Send {
    fn new(_page_config: SubpageConfig, _page_client: T, _max_iterations_num: i32) -> Self;
    async fn get_subpage_menu_items(&self) -> Result<Menu, Error>;
}
