use scraper::{Selector};
use async_trait::async_trait;
use reqwest::{Error};

use crate::clients::{
    kwestiasmaku_client::{KwestiasmakuClient}
};

use crate::providers::{
    provider::{SubpageDataProvider},
    kwestiasmaku_provider::{KwestiasmakuDataProvider}
};

use crate::models::{
    menu::{Menu, MenuItem},
    dish_type_enum::{DishType}
};
use super::subpage_config::{SubpageConfig};
use super::data_source::{DataSource};

pub struct KwestiasmakuDataSource {  
    client: KwestiasmakuClient,
    sub_pages: Vec<SubpageConfig>
}

#[async_trait]
impl DataSource for KwestiasmakuDataSource {
    fn new(_base_uri: &str) -> Self {
        let ks_client = KwestiasmakuClient::new(_base_uri);

        Self {
            client: ks_client,
            sub_pages: vec![

                SubpageConfig {
                    _relative_uri: String::from("/blog-kulinarny/category/dania-obiadowe"),
                    _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
                    _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(),
                    _subpage_dishes_category: DishType::DINNER
                },

                SubpageConfig {
                    _relative_uri: String::from("/dania_dla_dwojga/sniadania/przepisy.html"),
                    _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
                    _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(), 
                    _subpage_dishes_category: DishType::BREAKFAST
                }
            ]
        }
    }

    async fn get_menu_for_dish_type(&self, _dish_type: DishType) -> Result<Menu, Error> {
        let _sub_pages_for_chosen_dish_type: Vec<SubpageConfig> = 
            self.sub_pages
                .iter()
                .cloned()
                .filter(|_sub_page_config| _sub_page_config.is_for_dish_type(_dish_type))
                .collect();
        
        if _sub_pages_for_chosen_dish_type.is_empty() {
            return Ok(Menu {_dish_type: _dish_type, _dishes: vec![] });
        }

        let _sub_pages_providers: Vec<KwestiasmakuDataProvider<&KwestiasmakuClient>> = _sub_pages_for_chosen_dish_type
            .into_iter()
            .map(|_sub_page_config| 
                SubpageDataProvider::new(
                    _sub_page_config,
                    &self.client,
                    0
                )
            )
            .collect();

        if _sub_pages_providers.is_empty() {
            return Ok(Menu {_dish_type: _dish_type, _dishes: vec![] });
        }

        let mut _dishes: Vec<MenuItem> = vec![];
        for _each_provider in _sub_pages_providers {
            let _menu = _each_provider.get_subpage_menu_items().await?;
            _dishes.extend(_menu._dishes);
        }

        Ok(Menu {_dish_type: _dish_type, _dishes})
    }
}