use async_trait::async_trait;
use reqwest::{Error};
use regex::Regex;
use scraper::{Html};
use std::marker::{Sync, Send};

use super::provider::{PageDataProvider};

use crate::data_sources::page_config::{PageConfig};
use crate::clients::client::{Client};
use crate::models::menu::{MenuItem, Menu, Ingredient};

pub struct KwestiasmakuDataProvider <T> 
where T: Client {
    _page_config: PageConfig,
    _page_client: T,
    _max_iterations_num: i32 // 0 -> disables limit
}

#[async_trait]
impl <'a, T> PageDataProvider <T> for KwestiasmakuDataProvider <T> 
where T: Client + Sync + Send {
    fn new(_page_config: PageConfig, _page_client: T, _max_iterations_num: i32) -> Self { 
        Self {
            _page_config,
            _page_client,
            _max_iterations_num 
        }
    }

    async fn get_page_menu_items(&self) -> Result<Menu, Error> {
        let mut _page_menu_items: Vec<MenuItem> = vec![];

        let _a_value_selector = Regex::new(r">(.*?)</a>").unwrap();

        let mut _request_uri: String = self._page_config._relative_uri.to_owned();

        let mut _iteration = 0;
        loop {
            if self._max_iterations_num != 0 {
                if _iteration >= self._max_iterations_num {
                    break;
                }

                _iteration += 1;
            }

            println!("Fetching data from uri: {}", _request_uri);

            let body = self._page_client.get_subpage_html_body(&_request_uri).await?;
            let doc = Html::parse_document(&body);

            for element in doc.select(&self._page_config._menu_items_selector) {
                let _element_as_html = element.html();
                let _dish_relative_path = element.value().attr("href").unwrap();
                let _dish_name = _a_value_selector.captures(&_element_as_html).unwrap().get(1).unwrap().as_str();

                _page_menu_items.push(
                    MenuItem { 
                        _dish_name: _dish_name.to_owned(),
                        _dish_absolute_path: self._page_client.relative_path_to_full_uri(_dish_relative_path),
                        _dish_relative_path: _dish_relative_path.to_string(),
                        _ingredients: vec![] // ingredients list is build outside this function
                    }
                );
            }

            match doc.select(&self._page_config._next_page_selector).next() {
                Some(_next_page_uri) => {
                    match _next_page_uri.value().attr("href") {
                        Some(_next_page_uri) => _request_uri = _next_page_uri.to_owned(),
                        None => break
                    }
                },
                None => break
            }
        }

        Ok(Menu::new(
            self._page_config._sub_page_dishes_category,
            _page_menu_items
        ))
    }

    async fn get_menu_dishes_details(&self, mut _menu: Menu) -> Result<Menu, Error> {
        println!("\nGetting menu dishes details...\n");
        
        let _ingredient_value_selector = Regex::new(r"<li>\n\t\t(.+?)</li>").unwrap();

        for _each_menu_dish in &mut _menu._dishes {
            let mut _resource_details_uri: &str= &_each_menu_dish._dish_relative_path;

            println!("Getting details for dish: {:?} from URI {:?}\n", _each_menu_dish, _resource_details_uri);

            let _details_page_body = self._page_client.get_subpage_html_body(&_resource_details_uri).await?;
            let _details_doc = Html::parse_document(&_details_page_body);

            let mut _dish_ingredients: Vec<Ingredient> = vec![];
            for element in _details_doc.select(&self._page_config._sub_page_config._ingredients_selector) {
                let _element_as_html = element.html();
                let _dish = element.value();
                let _ingredient_description = _ingredient_value_selector
                    .captures(&_element_as_html)
                    .unwrap()
                    .get(1)
                    .unwrap()
                    .as_str()
                    // replacing html elements from fetched data
                    .replace("\\n", "")
                    .replace("\\t", "")
                    .replace("<strong>", "")
                    .replace(r"</strong>", "");
                
                _dish_ingredients.push(Ingredient {
                    _name: _ingredient_description,
                    _amount: 0_f32 //TODO: extract amount and unit from each element
                })
            }

            _each_menu_dish.update_with_ingredients(_dish_ingredients);
        }

        Ok(_menu)
    }
}
