use async_trait::async_trait;
use reqwest::{Error};
use regex::Regex;
use scraper::{Html};
use std::marker::{Sync, Send};

use super::provider::{SubpageDataProvider};

use crate::data_sources::subpage_config::{SubpageConfig};
use crate::clients::client::{Client};
use crate::models::menu::{MenuItem, Menu};

pub struct KwestiasmakuDataProvider <T> 
where T: Client {
    _page_config: SubpageConfig,
    _page_client: T,
    _max_iterations_num: i32 // 0 -> disables limit
}

#[async_trait]
impl <'a, T> SubpageDataProvider <T> for KwestiasmakuDataProvider <T> 
where T: Client + Sync + Send {
    fn new(_page_config: SubpageConfig, _page_client: T, _max_iterations_num: i32) -> Self { 
        Self {
            _page_config,
            _page_client,
            _max_iterations_num 
        }
    }

    async fn get_subpage_menu_items(&self) -> Result<Menu, Error> {
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

            println!("Processing from uri: {}", _request_uri);

            let body = self._page_client.get_subpage_html_body(&_request_uri).await?;
            let doc = Html::parse_document(&body);

            for element in doc.select(&self._page_config._menu_items_selector) {
                let _element_as_html = element.html();
                let _dish_relative_path = element.value().attr("href").unwrap();
                let _dish_name = _a_value_selector.captures(&_element_as_html).unwrap().get(1).unwrap().as_str();

                _page_menu_items.push(
                    MenuItem { 
                        _dish_name: _dish_name.to_owned(), 
                        _dish_path: self._page_client.relative_path_to_full_uri(_dish_relative_path)
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
            self._page_config._subpage_dishes_category,
            _page_menu_items
        ))
    }
}
