use scraper::{Selector};

struct SubpageConfig <'a> {
    _relative_uri: &'a str,
    _menu_items_selector: Selector,
    _next_page_selector: Selector
}
