use scraper::{Selector};

pub struct SubpageConfig {
    pub _relative_uri: String,
    pub _menu_items_selector: Selector,
    pub _next_page_selector: Selector
}
