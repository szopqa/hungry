
use scraper::{Selector};
use async_trait::async_trait;
use reqwest::{Error};

use crate::clients::{
    kwestiasmaku_client::{KwestiasmakuClient}
};

use crate::providers::{
    provider::{PageDataProvider},
    kwestiasmaku_provider::{KwestiasmakuDataProvider}
};

use crate::models::{
    menu::{Menu, MenuItem},
    dish_type_enum::{DishType}
};
use super::page_config::{
    PageConfig,
    SubPageConfig
};
use super::data_source::{DataSource};

pub struct KwestiasmakuDataSource {  
    client: KwestiasmakuClient,
    sub_pages: Vec<PageConfig>
}
