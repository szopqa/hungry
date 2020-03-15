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

#[async_trait]
impl DataSource for KwestiasmakuDataSource {
    fn new(_base_uri: &str) -> Self {
        let ks_client = KwestiasmakuClient::new(_base_uri);

        Self {
            client: ks_client,
            sub_pages: vec![

                // https://www.kwestiasmaku.com/blog-kulinarny/category/dania-obiadowe
                PageConfig {
                    _relative_uri: String::from("/blog-kulinarny/category/dania-obiadowe"),
                    _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
                    _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(),
                    _sub_page_config: SubPageConfig {
                        _ingredients_selector: Selector::parse(".field-type-text-long.field-label-hidden li").unwrap()
                    },
                    _sub_page_dishes_category: DishType::DINNER
                },

                // https://www.kwestiasmaku.com/dania_dla_dwojga/sniadania/przepisy.html
                PageConfig {
                    _relative_uri: String::from("/dania_dla_dwojga/sniadania/przepisy.html"),
                    _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
                    _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(), 
                    _sub_page_config: SubPageConfig {
                        _ingredients_selector: Selector::parse(".field-type-text-long.field-label-hidden li").unwrap()
                    },
                    _sub_page_dishes_category: DishType::BREAKFAST
                },

                // https://www.kwestiasmaku.com/pasta/pasta.html
                PageConfig {
                    _relative_uri: String::from("/pasta/pasta.html"),
                    _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
                    _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(), 
                    _sub_page_config: SubPageConfig {
                        _ingredients_selector: Selector::parse(".field-type-text-long.field-label-hidden li").unwrap()
                    },
                    _sub_page_dishes_category: DishType::DINNER
                },

                // https://www.kwestiasmaku.com/blog-kulinarny/category/przepisy-fit?sort_by=created&f[]=field_przepisy:976&default_filter=803
                PageConfig {
                    _relative_uri: String::from("/blog-kulinarny/category/przepisy-fit?sort_by=created&f[]=field_przepisy:976&default_filter=803"),
                    _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
                    _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(), 
                    _sub_page_config: SubPageConfig {
                        _ingredients_selector: Selector::parse(".field-type-text-long.field-label-hidden li").unwrap()
                    },
                    _sub_page_dishes_category: DishType::DINNER
                },

                // https://www.kwestiasmaku.com/przepisy/lunche-do-pracy 
                PageConfig {
                    _relative_uri: String::from("/przepisy/lunche-do-pracy"),
                    _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
                    _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(), 
                    _sub_page_config: SubPageConfig {
                        _ingredients_selector: Selector::parse(".field-type-text-long.field-label-hidden li").unwrap()
                    },
                    _sub_page_dishes_category: DishType::LUNCH
                }
            ]
        }
    }

    async fn get_menu_for_dish_type(&self, _dish_type: DishType) -> Result<Menu, Error> {
        let _sub_pages_for_chosen_dish_type: Vec<PageConfig> = 
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
                PageDataProvider::new(
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
            let _menu = _each_provider.get_page_menu_items().await?;
            _dishes.extend(_menu._dishes);
        }

        Ok(Menu {_dish_type: _dish_type, _dishes})
    }

    async fn get_ingredients_for_menu(&self, _menu: Menu) -> Result<Menu, Error> {
        let _page_configs: Vec<PageConfig> = 
            self.sub_pages
                .iter()
                .cloned()
                .filter(|_sub_page_config| _sub_page_config.is_for_dish_type(_menu._dish_type))
                .collect();

        // TODO: Fix getting only one config from collection
        let _sub_page_config: PageConfig= 
            _page_configs
                .get(0)
                .unwrap()
                .clone();

        println!("{:?}", _sub_page_config);

        let _sub_page_details_provider: KwestiasmakuDataProvider<&KwestiasmakuClient> = PageDataProvider::new(
            _sub_page_config,
            &self.client,
            0
        );

        let _menu_with_dishes_details = _sub_page_details_provider.get_menu_dishes_details(_menu).await?;

        Ok(_menu_with_dishes_details)
    }
}