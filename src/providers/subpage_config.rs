use scraper::{Selector};

use crate::models::dish_type_enum::{DishType};

pub struct SubpageConfig {
    pub _relative_uri: String,
    pub _menu_items_selector: Selector,
    pub _next_page_selector: Selector,
    pub _subpage_dishes_category: DishType
}
