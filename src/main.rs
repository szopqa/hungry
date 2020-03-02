use reqwest::{Error};
use scraper::{Html, Selector};
use regex::Regex;

mod clients;
use clients::kwestiasmaku_client::{KwestiasmakuClient};
use clients::client::{Client};

// mod providers;
// use providers::kwestiasmaku_provider::{KwestiasmakuDataProvider};

#[derive(Debug)]
struct MenuItem {
    _dish_name: String,
    _dish_relative_path: String
}

struct PageConfig <'a> {
    _relative_uri: &'a str,
    _menu_items_selector: Selector,
    _next_page_selector: Selector
}

struct PageMenuProvider <'a, T> 
where T: Client {
    _page_config: &'a PageConfig<'a>,
    _page_client: T,
    _max_iterations_num: i32 // 0 -> disables limit
}

impl <'a, T> PageMenuProvider <'a, T> 
where T: Client {
    pub fn new(_page_config: &'a PageConfig, _page_client: T, _max_iterations_num: i32) -> Self { 
        Self {
            _page_config,
            _page_client,
            _max_iterations_num 
        }
    }

    pub async fn get_menu_items(&self) -> Result<Vec<MenuItem>, Error> {
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
                        _dish_relative_path: _dish_relative_path.to_owned() 
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

        Ok(_page_menu_items)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _home_uri = "https://www.kwestiasmaku.com";

    let _breakfast_config = PageConfig {
        _relative_uri: "/dania_dla_dwojga/sniadania/przepisy.html",
        _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
        _next_page_selector: Selector::parse("#block-system-main .last a").unwrap()
    };

    let _main_dishes_config = PageConfig {
        _relative_uri: "/blog-kulinarny/category/dania-obiadowe",
        _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
        _next_page_selector: Selector::parse("#block-system-main .last a").unwrap()
    };

    let _breakfast_menu_provider = PageMenuProvider::new(
        &_breakfast_config,
        KwestiasmakuClient::new(&_home_uri),
        0
    );

    let _main_dishes_menu_provider = PageMenuProvider::new(
        &_main_dishes_config,
        KwestiasmakuClient::new(&_home_uri),
        0
    );

    let _breakfast_menu_items = _breakfast_menu_provider.get_menu_items().await?;
    let _dinner_menu_items = _main_dishes_menu_provider.get_menu_items().await?;
    println!("{:?}", _breakfast_menu_items);
    println!("{:?}", _dinner_menu_items);

    Ok(())
}
