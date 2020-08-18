use std::sync::Arc;
use async_trait::async_trait;
use reqwest::{Error};
use std::marker::{Sync, Send};

use crate::clients::client::{Client};
use crate::models::menu::{Menu, MenuItem};
use crate::data_sources::page_config::{PageConfig};

#[async_trait]
pub trait PageDataProvider<T> where T: Client + Sync + Send {
    fn new(_page_config: PageConfig, _page_client: Arc<T>, _max_iterations_num: i32) -> Self;
    async fn get_page_menu_items(&self) -> Result<Menu, Error>;
    async fn get_menu_dishes_details(&self, _menu: Menu) -> Result<Menu, Error>;
    async fn get_ingredients_for_dish<'a>(&self, _dish: &'a mut MenuItem) -> Result<&'a MenuItem, Error>;
}
