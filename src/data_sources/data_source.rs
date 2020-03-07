use async_trait::async_trait;
use reqwest::{Error};

use crate::models::{
    menu::{Menu},
    dish_type_enum::{DishType}
};

#[async_trait]
pub trait DataSource {
    fn new(_base_uri: &str) -> Self;
    async fn get_menu_for_dish_type(&self, _dish_type: DishType) -> Result<Menu, Error>;
}
