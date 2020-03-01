use scraper::{Html, Selector};
use regex::Regex;

mod clients;
use clients::kwestiasmaku_client::{KwestiasmakuClient};
use clients::client::{Client};

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

    pub fn get_menu_items(&self, _max_items_num: i32) -> Vec<MenuItem> {
        vec![]
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _home_uri = "https://www.kwestiasmaku.com";
    let _main_dishes_config = PageConfig {
        _relative_uri: "/blog-kulinarny/category/dania-obiadowe",
        _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
        _next_page_selector: Selector::parse(".views-field-title a").unwrap() // Temporary
    };

    let client = KwestiasmakuClient::new(&_home_uri);

    println!("{:?}", client);
//    let _main_dishes_menu_provider = PageMenuProvider::new(
//        &_main_dishes_config,
//        client,
//        0
//    );

    let _main_dishes_uri = "/blog-kulinarny/category/dania-obiadowe";
    let body = client.get_subpage_html_body(_main_dishes_uri).await?;

    let doc = Html::parse_document(&body);

    let selector = Selector::parse(".views-field-title a").unwrap();

    let mut _page_menu_items: Vec<MenuItem> = vec![];

    let _a_value_selector = Regex::new(r">(.*?)</a>").unwrap();
    for element in doc.select(&selector) {
        let _element_as_html = element.html();
        let _dish_relative_path = element.value().attr("href").unwrap();

        let _dish_name = _a_value_selector.captures(&_element_as_html).unwrap().get(1).unwrap().as_str();
        _page_menu_items.push(MenuItem { _dish_name: _dish_name.to_owned(), _dish_relative_path: _dish_relative_path.to_owned() })
    }

    println!("{:?}", _page_menu_items);

    Ok(())
}
