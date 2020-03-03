use scraper::{Selector};

mod clients;
use clients::{
    kwestiasmaku_client::{KwestiasmakuClient},
    client::{Client}
};

mod providers;
use providers::{
    kwestiasmaku_provider::{KwestiasmakuDataProvider},
    subpage_config::{SubpageConfig},
    provider::{SubpageDataProvider}
};

mod models;
use models::{
    dish_type_enum::{DishType}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ks_client = KwestiasmakuClient::new("https://www.kwestiasmaku.com");

    let _breakfast_config = SubpageConfig {
        _relative_uri: String::from("/dania_dla_dwojga/sniadania/przepisy.html"),
        _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
        _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(), 
        _subpage_dishes_category: DishType::BREAKFAST
    };

    let _main_dishes_config = SubpageConfig {
        _relative_uri: String::from("/blog-kulinarny/category/dania-obiadowe"),
        _menu_items_selector: Selector::parse(".views-field-title a").unwrap(),
        _next_page_selector: Selector::parse("#block-system-main .last a").unwrap(),
        _subpage_dishes_category: DishType::DINNER
    };


    let _breakfast_menu_provider = KwestiasmakuDataProvider::new(
        _breakfast_config,
        &ks_client,
        0
    );

    let _main_dishes_menu_provider = KwestiasmakuDataProvider::new(
        _main_dishes_config,
        &ks_client,
        0
    );

    let _breakfast_menu_items = _breakfast_menu_provider.get_subpage_menu_items().await?;
    let _dinner_menu_items = _main_dishes_menu_provider.get_subpage_menu_items().await?;
    println!("{:?}", _breakfast_menu_items);
    println!("{:?}", _dinner_menu_items);

    Ok(())
}
